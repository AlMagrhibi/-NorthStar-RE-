//! Ordinary foreground input. Text is never echoed into output or audit metadata.
use super::*;
#[link(name = "kernel32")]
unsafe extern "system" {
    fn CreateMutexW(attributes: *const std::ffi::c_void, owned: i32, name: *const u16) -> isize;
    fn WaitForSingleObject(handle: isize, millis: u32) -> u32;
    fn ReleaseMutex(handle: isize) -> i32;
    fn CloseHandle(handle: isize) -> i32;
}
#[link(name = "user32")]
unsafe extern "system" {
    fn GetAsyncKeyState(key: i32) -> i16;
    fn SetThreadDpiAwarenessContext(context: isize) -> isize;
}
pub(super) struct DpiContext(isize);
impl DpiContext {
    pub(super) fn new() -> io::Result<Self> {
        let previous = unsafe { SetThreadDpiAwarenessContext(-4) };
        if previous == 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(Self(previous))
    }
}
impl Drop for DpiContext {
    fn drop(&mut self) {
        unsafe {
            SetThreadDpiAwarenessContext(self.0);
        }
    }
}
pub(super) struct DesktopLock(isize);
impl DesktopLock {
    pub(super) fn acquire() -> io::Result<Self> {
        let name: Vec<u16> = "Local\\NorthStarRE.Desktop.Actions"
            .encode_utf16()
            .chain([0])
            .collect();
        let handle = unsafe { CreateMutexW(std::ptr::null(), 0, name.as_ptr()) };
        if handle == 0 {
            return Err(io::Error::last_os_error());
        }
        match unsafe { WaitForSingleObject(handle, 2000) } {
            0 | 0x80 => Ok(Self(handle)),
            _ => {
                unsafe {
                    CloseHandle(handle);
                }
                Err(invalid("another desktop action is active"))
            }
        }
    }
}
impl Drop for DesktopLock {
    fn drop(&mut self) {
        unsafe {
            ReleaseMutex(self.0);
            CloseHandle(self.0);
        }
    }
}
pub(super) fn guard(hwnd: Hwnd, pid: u32) -> io::Result<()> {
    identity(hwnd, Some(pid))?;
    if unsafe { foreground_window() != hwnd || is_iconic(hwnd) != 0 } {
        return Err(invalid(
            "selected window must remain foreground and restored",
        ));
    }
    if unsafe { GetAsyncKeyState(0x1b) } < 0 {
        return Err(io::Error::new(
            io::ErrorKind::Interrupted,
            "Escape held; input cancelled",
        ));
    }
    if [1, 2, 4, 0x10, 0x11, 0x12, 0x5b, 0x5c]
        .iter()
        .any(|&key| unsafe { GetAsyncKeyState(key) } < 0)
    {
        return Err(io::Error::new(
            io::ErrorKind::Interrupted,
            "physical button/modifier held; input cancelled",
        ));
    }
    Ok(())
}
fn event(vk: u16, scan: u16, flags: u32) -> Input {
    Input {
        kind: 1,
        data: InputData {
            keyboard: KeyboardInput {
                vk,
                scan,
                flags,
                time: 0,
                extra_info: 0,
            },
        },
    }
}
fn send(events: &[Input]) -> usize {
    unsafe {
        send_input(
            events.len() as u32,
            events.as_ptr(),
            std::mem::size_of::<Input>() as i32,
        ) as usize
    }
}
pub(super) fn text(hwnd: Hwnd, pid: u32, text: &str) -> io::Result<bool> {
    let units: Vec<u16> = text.encode_utf16().collect();
    if units.is_empty() || units.len() > 1024 || text.contains('\0') {
        return Err(invalid(
            "text must contain 1..=1024 UTF-16 units and no NUL",
        ));
    }
    let mut inserted = 0;
    let mut start = 0;
    while start < units.len() {
        let mut end = (start + 32).min(units.len());
        if end < units.len() && (0xd800..=0xdbff).contains(&units[end - 1]) {
            end -= 1;
        }
        let chunk = &units[start..end];
        guard(hwnd, pid)?;
        let events: Vec<Input> = chunk
            .iter()
            .flat_map(|&u| [event(0, u, 4), event(0, u, 6)])
            .collect();
        let sent = send(&events);
        inserted += sent;
        if sent != events.len() {
            // Release the last Unicode key if insertion stopped on its key-down.
            if sent % 2 == 1 {
                send(&[event(0, chunk[sent / 2], 6)]);
            }
            break;
        }
        start = end;
    }
    let dispatched = inserted == units.len() * 2;
    println!(
        "{}",
        json!({"schema":1,"operation":"type","hwnd":hwnd.to_string(),"pid":pid,"dispatched":dispatched,"utf16_units":units.len(),"events_inserted":inserted,"text_logged":false,"application_effect_verified":false})
    );
    Ok(dispatched)
}
fn chord(spec: &str) -> io::Result<Vec<u16>> {
    let parts: Vec<_> = spec.split('+').collect();
    if parts.is_empty() || parts.len() > 4 {
        return Err(invalid("invalid key chord"));
    }
    let mut keys = Vec::new();
    for (i, part) in parts.iter().enumerate() {
        let value = match part.to_ascii_uppercase().as_str() {
            "CTRL" => 0x11,
            "SHIFT" => 0x10,
            "ALT" => 0x12,
            "ENTER" => 0x0d,
            "TAB" => 9,
            "ESCAPE" => 0x1b,
            "SPACE" => 0x20,
            "BACKSPACE" => 8,
            "DELETE" => 0x2e,
            "LEFT" => 0x25,
            "UP" => 0x26,
            "RIGHT" => 0x27,
            "DOWN" => 0x28,
            "HOME" => 0x24,
            "END" => 0x23,
            "PAGEUP" => 0x21,
            "PAGEDOWN" => 0x22,
            s if s.len() == 1
                && s.bytes()
                    .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit()) =>
            {
                s.as_bytes()[0] as u16
            }
            _ => {
                return Err(invalid(
                    "unsupported key; use named keys or ASCII letters/digits",
                ))
            }
        };
        if keys.contains(&value)
            || (i + 1 < parts.len() && !matches!(value, 0x10..=0x12))
            || (i + 1 == parts.len() && matches!(value, 0x10..=0x12))
        {
            return Err(invalid("invalid or duplicate modifier"));
        }
        keys.push(value);
    }
    Ok(keys)
}
pub(super) fn key(hwnd: Hwnd, pid: u32, spec: &str) -> io::Result<bool> {
    let keys = chord(spec)?;
    guard(hwnd, pid)?;
    let events: Vec<Input> = keys
        .iter()
        .map(|&k| event(k, 0, 0))
        .chain(keys.iter().rev().map(|&k| event(k, 0, 2)))
        .collect();
    let sent = send(&events);
    if sent != events.len() {
        let release: Vec<_> = keys.iter().rev().map(|&k| event(k, 0, 2)).collect();
        send(&release);
    }
    println!(
        "{}",
        json!({"schema":1,"operation":"key","hwnd":hwnd.to_string(),"pid":pid,"dispatched":sent==events.len(),"events_inserted":sent,"application_effect_verified":false})
    );
    Ok(sent == events.len())
}
pub(super) fn pointer(hwnd: Hwnd, pid: u32, x: i32, y: i32) -> io::Result<bool> {
    guard(hwnd, pid)?;
    let mut r = Rect::default();
    if unsafe { get_client_rect(hwnd, &mut r) } == 0
        || x < 0
        || y < 0
        || x >= r.right
        || y >= r.bottom
    {
        return Err(invalid("point outside client area"));
    }
    let mut point = Point { x, y };
    if unsafe { client_to_screen(hwnd, &mut point) } == 0 {
        return Err(io::Error::last_os_error());
    }
    if unsafe { get_ancestor(window_from_point(point), GA_ROOT) } != hwnd {
        return Err(invalid("selected point is covered"));
    }
    guard(hwnd, pid)?;
    if unsafe { set_cursor(point.x, point.y) } == 0 {
        return Err(io::Error::last_os_error());
    }
    let mut actual = Point::default();
    let verified =
        unsafe { get_cursor(&mut actual) } != 0 && actual.x == point.x && actual.y == point.y;
    println!(
        "{}",
        json!({"schema":1,"operation":"pointer","verified":verified,"hwnd":hwnd.to_string(),"pid":pid,"client_x":x,"client_y":y})
    );
    Ok(verified)
}
pub(super) fn cancelled() -> bool {
    unsafe { GetAsyncKeyState(0x1b) < 0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validates_chords_without_input() {
        assert_eq!(chord("Ctrl+A").unwrap(), vec![0x11, 0x41]);
        for s in ["Ctrl", "Ctrl+Ctrl+A", "Enter+A", "WIN+R", "", "A+B"] {
            assert!(chord(s).is_err(), "{s}");
        }
        assert_eq!(std::mem::size_of::<KeyboardInput>(), 24);
        assert_eq!(std::mem::size_of::<Input>(), 40);
        assert!(text(0, 1, "abc").is_err());
    }
}
