//! Selected-window wheel and bounded drag; release the held button on every exit path.
use super::*;
fn point(hwnd: Hwnd, x: i32, y: i32) -> io::Result<Point> {
    let mut r = Rect::default();
    if unsafe { get_client_rect(hwnd, &mut r) } == 0
        || x < 0
        || y < 0
        || x >= r.right
        || y >= r.bottom
    {
        return Err(invalid("mouse point outside client area"));
    }
    let mut p = Point { x, y };
    if unsafe { client_to_screen(hwnd, &mut p) } == 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(p)
}
fn owns_point(hwnd: Hwnd, pid: u32, p: Point) -> io::Result<()> {
    identity(hwnd, Some(pid))?;
    if keyboard::cancelled()
        || unsafe {
            foreground_window() != hwnd || get_ancestor(window_from_point(p), GA_ROOT) != hwnd
        }
    {
        return Err(invalid("mouse target changed, covered or cancelled"));
    }
    Ok(())
}
fn send(event: &Input) -> bool {
    unsafe { send_input(1, event, std::mem::size_of::<Input>() as i32) == 1 }
}
fn cursor_matches(point: Point) -> io::Result<()> {
    let mut actual = Point::default();
    if unsafe { get_cursor(&mut actual) } == 0 || actual.x != point.x || actual.y != point.y {
        return Err(invalid("cursor changed or clipped; mouse action aborted"));
    }
    Ok(())
}
pub(super) fn scroll(hwnd: Hwnd, pid: u32, x: i32, y: i32, delta: i32) -> io::Result<bool> {
    if delta == 0 || !(-1200..=1200).contains(&delta) || delta % 120 != 0 {
        return Err(invalid(
            "wheel delta must be a nonzero multiple of 120 within -1200..=1200",
        ));
    }
    keyboard::guard(hwnd, pid)?;
    let p = point(hwnd, x, y)?;
    owns_point(hwnd, pid, p)?;
    if unsafe { set_cursor(p.x, p.y) } == 0 {
        return Err(io::Error::last_os_error());
    }
    owns_point(hwnd, pid, p)?;
    cursor_matches(p)?;
    let mut event = mouse_input(0x0800);
    event.data.mouse.mouse_data = delta as u32;
    let sent = send(&event);
    println!(
        "{}",
        json!({"schema":1,"operation":"scroll","hwnd":hwnd.to_string(),"pid":pid,"dispatched":sent,"delta":delta,"application_effect_verified":false})
    );
    Ok(sent)
}
struct Release(bool);
impl Release {
    fn finish(mut self) -> bool {
        let sent = send(&mouse_input(MOUSEEVENTF_LEFTUP));
        self.0 = !sent;
        sent
    }
}
impl Drop for Release {
    fn drop(&mut self) {
        if self.0 {
            send(&mouse_input(MOUSEEVENTF_LEFTUP));
        }
    }
}
pub(super) fn drag(hwnd: Hwnd, pid: u32, start: (i32, i32), end: (i32, i32)) -> io::Result<bool> {
    keyboard::guard(hwnd, pid)?;
    let a = point(hwnd, start.0, start.1)?;
    let b = point(hwnd, end.0, end.1)?;
    owns_point(hwnd, pid, a)?;
    owns_point(hwnd, pid, b)?;
    if unsafe { set_cursor(a.x, a.y) } == 0 {
        return Err(io::Error::last_os_error());
    }
    owns_point(hwnd, pid, a)?;
    cursor_matches(a)?;
    if !send(&mouse_input(MOUSEEVENTF_LEFTDOWN)) {
        return Err(invalid("drag button insertion refused"));
    }
    let release = Release(true);
    let mut previous = a;
    for step in 1..=16 {
        let mut actual = Point::default();
        if unsafe { get_cursor(&mut actual) } == 0
            || actual.x != previous.x
            || actual.y != previous.y
        {
            return Err(invalid("cursor changed during drag; release attempted"));
        }
        let p = Point {
            x: a.x + ((b.x as i64 - a.x as i64) * step / 16) as i32,
            y: a.y + ((b.y as i64 - a.y as i64) * step / 16) as i32,
        };
        owns_point(hwnd, pid, p)?;
        if unsafe { set_cursor(p.x, p.y) } == 0 {
            return Err(io::Error::last_os_error());
        }
        previous = p;
        thread::sleep(Duration::from_millis(10));
    }
    owns_point(hwnd, pid, b)?;
    cursor_matches(b)?;
    let released = release.finish();
    println!(
        "{}",
        json!({"schema":1,"operation":"drag","hwnd":hwnd.to_string(),"pid":pid,"dispatched":released,"button_release_inserted":released,"application_effect_verified":false})
    );
    Ok(released)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn invalid_wheel_and_stale_drag_are_rejected() {
        assert!(scroll(0, 1, 0, 0, 1).is_err());
        assert!(scroll(0, 1, 0, 0, 0).is_err());
        assert!(drag(0, 1, (0, 0), (1, 1)).is_err());
    }
}
