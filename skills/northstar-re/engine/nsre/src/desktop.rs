//! Explicit window management on the caller's interactive Windows desktop.
use serde_json::{json, Value};
use std::{ffi::OsString, io, thread, time::Duration};

type Hwnd = isize;
const MAX_WINDOWS: usize = 512;
const GA_ROOT: u32 = 2;
const SW_RESTORE: i32 = 9;
const SWP_NOSIZE: u32 = 0x0001;
const SWP_NOMOVE: u32 = 0x0002;
const SWP_NOZORDER: u32 = 0x0004;
const SWP_NOACTIVATE: u32 = 0x0010;
const SWP_ASYNCWINDOWPOS: u32 = 0x4000;

#[repr(C)]
#[derive(Default)]
struct Rect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

#[link(name = "user32")]
unsafe extern "system" {
    #[link_name = "EnumWindows"]
    fn enum_windows(callback: extern "system" fn(Hwnd, isize) -> i32, data: isize) -> i32;
    #[link_name = "GetWindowThreadProcessId"]
    fn window_pid(hwnd: Hwnd, pid: *mut u32) -> u32;
    #[link_name = "IsWindow"]
    fn is_window(hwnd: Hwnd) -> i32;
    #[link_name = "IsWindowVisible"]
    fn is_visible(hwnd: Hwnd) -> i32;
    #[link_name = "IsIconic"]
    fn is_iconic(hwnd: Hwnd) -> i32;
    #[link_name = "IsZoomed"]
    fn is_zoomed(hwnd: Hwnd) -> i32;
    #[link_name = "GetAncestor"]
    fn get_ancestor(hwnd: Hwnd, flags: u32) -> Hwnd;
    #[link_name = "GetDesktopWindow"]
    fn desktop_window() -> Hwnd;
    #[link_name = "GetShellWindow"]
    fn shell_window() -> Hwnd;
    #[link_name = "GetForegroundWindow"]
    fn foreground_window() -> Hwnd;
    #[link_name = "GetWindowRect"]
    fn get_rect(hwnd: Hwnd, rect: *mut Rect) -> i32;
    #[link_name = "GetWindowTextW"]
    fn get_text(hwnd: Hwnd, buffer: *mut u16, capacity: i32) -> i32;
    #[link_name = "ShowWindowAsync"]
    fn show_async(hwnd: Hwnd, command: i32) -> i32;
    #[link_name = "SetForegroundWindow"]
    fn set_foreground(hwnd: Hwnd) -> i32;
    #[link_name = "SetWindowPos"]
    fn set_pos(hwnd: Hwnd, after: Hwnd, x: i32, y: i32, width: i32, height: i32, flags: u32)
        -> i32;
}

fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message)
}

fn eligible(hwnd: Hwnd) -> bool {
    // Only ordinary visible top-level windows; covered/minimized windows may qualify.
    unsafe {
        hwnd != 0
            && hwnd != desktop_window()
            && hwnd != shell_window()
            && is_window(hwnd) != 0
            && is_visible(hwnd) != 0
            && get_ancestor(hwnd, GA_ROOT) == hwnd
    }
}

fn identity(hwnd: Hwnd, expected: Option<u32>) -> io::Result<u32> {
    let mut pid = 0;
    if !eligible(hwnd) || unsafe { window_pid(hwnd, &mut pid) } == 0 || pid == 0 {
        return Err(invalid("window unavailable or outside desktop scope"));
    }
    if expected.is_some_and(|value| value != pid) {
        return Err(invalid("window owner PID changed"));
    }
    Ok(pid)
}

fn snapshot(hwnd: Hwnd, expected: Option<u32>) -> io::Result<Value> {
    let pid = identity(hwnd, expected)?;
    let mut rect = Rect::default();
    if unsafe { get_rect(hwnd, &mut rect) } == 0 {
        return Err(io::Error::last_os_error());
    }
    let mut title = [0u16; 256];
    let length = unsafe { get_text(hwnd, title.as_mut_ptr(), title.len() as i32) };
    let title = String::from_utf16_lossy(&title[..length.clamp(0, 255) as usize]);
    let (minimized, maximized, foreground) = unsafe {
        (
            is_iconic(hwnd) != 0,
            is_zoomed(hwnd) != 0,
            foreground_window() == hwnd,
        )
    };
    identity(hwnd, Some(pid))?;
    Ok(json!({
        "hwnd":hwnd.to_string(),"pid":pid,"title":title,
        "minimized":minimized,"maximized":maximized,"foreground":foreground,
        "x":rect.left,"y":rect.top,
        "width":rect.right.saturating_sub(rect.left),
        "height":rect.bottom.saturating_sub(rect.top)
    }))
}

struct Enumeration {
    handles: Vec<Hwnd>,
    truncated: bool,
}

extern "system" fn collect(hwnd: Hwnd, data: isize) -> i32 {
    // EnumWindows invokes this synchronously with our live context.
    let context = unsafe { &mut *(data as *mut Enumeration) };
    if eligible(hwnd) {
        if context.handles.len() < MAX_WINDOWS {
            context.handles.push(hwnd);
        } else {
            context.truncated = true;
        }
    }
    1
}

fn list() -> io::Result<Value> {
    let mut context = Enumeration {
        handles: Vec::with_capacity(MAX_WINDOWS),
        truncated: false,
    };
    if unsafe { enum_windows(collect, &mut context as *mut Enumeration as isize) } == 0 {
        return Err(io::Error::last_os_error());
    }
    let mut windows = Vec::new();
    let mut skipped = 0;
    for hwnd in context.handles {
        match snapshot(hwnd, None) {
            Ok(window) => windows.push(window),
            Err(_) => skipped += 1,
        }
    }
    Ok(json!({
        "schema":1,"scope":"caller desktop, visible top-level windows",
        "coordinate_space":"GetWindowRect; DPI virtualization may apply",
        "complete":!context.truncated && skipped == 0,
        "skipped":skipped,"truncated":context.truncated,"windows":windows
    }))
}

#[derive(Clone, Copy)]
enum Action {
    Restore,
    Focus,
    Move(i32, i32),
    Resize(i32, i32),
}

impl Action {
    fn matches(self, value: &Value) -> bool {
        match self {
            Self::Restore => value["minimized"] == false && value["maximized"] == false,
            Self::Focus => value["foreground"] == true,
            Self::Move(x, y) => value["x"] == x && value["y"] == y,
            Self::Resize(w, h) => value["width"] == w && value["height"] == h,
        }
    }
}

fn apply(hwnd: Hwnd, pid: u32, action: Action) -> io::Result<bool> {
    let before = snapshot(hwnd, Some(pid))?;
    if matches!(action, Action::Move(..) | Action::Resize(..))
        && (before["minimized"] == true || before["maximized"] == true)
    {
        return Err(invalid("restore window before moving or resizing"));
    }
    if matches!(action, Action::Focus) && before["minimized"] == true {
        return Err(invalid("restore window before requesting focus"));
    }
    identity(hwnd, Some(pid))?;
    let flags = SWP_NOZORDER | SWP_NOACTIVATE | SWP_ASYNCWINDOWPOS;
    let accepted = if action.matches(&before) {
        true
    } else {
        unsafe {
            match action {
                Action::Restore => show_async(hwnd, SW_RESTORE) != 0,
                Action::Focus => set_foreground(hwnd) != 0,
                Action::Move(x, y) => set_pos(hwnd, 0, x, y, 0, 0, flags | SWP_NOSIZE) != 0,
                Action::Resize(w, h) => set_pos(hwnd, 0, 0, 0, w, h, flags | SWP_NOMOVE) != 0,
            }
        }
    };
    let mut after = snapshot(hwnd, Some(pid))?;
    for _ in 0..20 {
        if !accepted || action.matches(&after) {
            break;
        }
        thread::sleep(Duration::from_millis(50));
        after = snapshot(hwnd, Some(pid))?;
    }
    let verified = accepted && action.matches(&after);
    println!(
        "{}",
        json!({"schema":1,"accepted":accepted,"verified":verified,
            "before":before,"after":after})
    );
    Ok(verified)
}

fn number<T: std::str::FromStr>(value: &OsString) -> io::Result<T> {
    value
        .to_str()
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| invalid("invalid numeric argument"))
}

fn dimensions(w: i32, h: i32) -> io::Result<Action> {
    if !(64..=16384).contains(&w) || !(64..=16384).contains(&h) {
        return Err(invalid("window dimensions must be within 64..=16384"));
    }
    Ok(Action::Resize(w, h))
}

pub fn command(args: impl Iterator<Item = OsString>) -> io::Result<bool> {
    let args: Vec<_> = args.collect();
    if args.len() == 1 && args[0] == "list" {
        println!("{}", list()?);
        return Ok(true);
    }
    if args.len() < 3 {
        return Err(invalid(
            "desktop list | inspect HWND PID | restore/focus HWND PID --apply | move/resize HWND PID A B --apply",
        ));
    }
    let hwnd: Hwnd = number(&args[1])?;
    let pid: u32 = number(&args[2])?;
    if hwnd == 0 || pid == 0 {
        return Err(invalid("window handle and owner PID required"));
    }
    if args.len() == 3 && args[0] == "inspect" {
        println!("{}", snapshot(hwnd, Some(pid))?);
        return Ok(true);
    }
    let action = match args[0].to_str() {
        Some("restore") if args.len() == 4 && args[3] == "--apply" => Action::Restore,
        Some("focus") if args.len() == 4 && args[3] == "--apply" => Action::Focus,
        Some("move") if args.len() == 6 && args[5] == "--apply" => {
            let x = number(&args[3])?;
            let y = number(&args[4])?;
            if !(-32768..=32768).contains(&x) || !(-32768..=32768).contains(&y) {
                return Err(invalid("window coordinate out of range"));
            }
            Action::Move(x, y)
        }
        Some("resize") if args.len() == 6 && args[5] == "--apply" => {
            dimensions(number(&args[3])?, number(&args[4])?)?
        }
        _ => return Err(invalid("invalid action or missing --apply")),
    };
    apply(hwnd, pid, action)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_invalid_target_and_geometry() {
        assert!(identity(0, Some(1)).is_err());
        assert!(dimensions(0, 400).is_err());
        assert!(dimensions(500, 18000).is_err());
        assert!(dimensions(800, 600).is_ok());
        assert!(!Action::Focus.matches(&json!({"foreground":false})));
        assert!(Action::Move(-200, 30).matches(&json!({"x":-200,"y":30})));
        assert!(!Action::Restore.matches(&json!({"minimized":false,"maximized":true})));
    }

    #[test]
    fn mutation_requires_explicit_apply() {
        let args = ["move", "1", "1", "100", "100"];
        assert!(command(args.into_iter().map(OsString::from)).is_err());
    }
}
