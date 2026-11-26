//! PrintWindow fallback, isolated in an owned child with a five-second deadline.
use super::*;
use std::{
    fs::{self, OpenOptions},
    path::Path,
    process::{Command, Stdio},
    time::Instant,
};
#[repr(C)]
struct BitmapInfo {
    size: u32,
    width: i32,
    height: i32,
    planes: u16,
    bits: u16,
    compression: u32,
    image_size: u32,
    x_ppm: i32,
    y_ppm: i32,
    colors: u32,
    important: u32,
}
#[link(name = "user32")]
unsafe extern "system" {
    fn GetDC(hwnd: Hwnd) -> isize;
    fn ReleaseDC(hwnd: Hwnd, dc: isize) -> i32;
    fn PrintWindow(hwnd: Hwnd, dc: isize, flags: u32) -> i32;
    fn GetWindowDisplayAffinity(hwnd: Hwnd, affinity: *mut u32) -> i32;
}
#[link(name = "gdi32")]
unsafe extern "system" {
    fn CreateCompatibleDC(dc: isize) -> isize;
    fn DeleteDC(dc: isize) -> i32;
    fn CreateDIBSection(
        dc: isize,
        info: *const BitmapInfo,
        usage: u32,
        bits: *mut *mut std::ffi::c_void,
        section: isize,
        offset: u32,
    ) -> isize;
    fn SelectObject(dc: isize, object: isize) -> isize;
    fn DeleteObject(object: isize) -> i32;
    fn GdiFlush() -> i32;
}
pub(super) fn dimensions(w: i64, h: i64) -> io::Result<usize> {
    if !(1..=8192).contains(&w) || !(1..=8192).contains(&h) || w * h > 16_777_216 {
        return Err(invalid(
            "capture exceeds 16 megapixels or geometry is invalid",
        ));
    }
    Ok((w * h * 4) as usize)
}
struct Surface {
    screen: isize,
    dc: isize,
    bitmap: isize,
    previous: isize,
}
impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            if self.previous != 0 && self.previous != -1 {
                SelectObject(self.dc, self.previous);
            }
            if self.bitmap != 0 {
                DeleteObject(self.bitmap);
            }
            if self.dc != 0 {
                DeleteDC(self.dc);
            }
            if self.screen != 0 {
                ReleaseDC(0, self.screen);
            }
        }
    }
}
fn write_frame(hwnd: Hwnd, pid: u32, path: &Path) -> io::Result<()> {
    let before = snapshot(hwnd, Some(pid))?;
    if before["minimized"] == true {
        return Err(invalid("restore minimized window before capture"));
    }
    let mut affinity = 0;
    if unsafe { GetWindowDisplayAffinity(hwnd, &mut affinity) } != 0 && affinity != 0 {
        return Err(invalid("window excludes capture"));
    }
    let w = before["width"].as_i64().unwrap();
    let h = before["height"].as_i64().unwrap();
    let count = dimensions(w, h)?;
    let mut surface = Surface {
        screen: unsafe { GetDC(0) },
        dc: 0,
        bitmap: 0,
        previous: 0,
    };
    if surface.screen == 0 {
        return Err(io::Error::last_os_error());
    }
    surface.dc = unsafe { CreateCompatibleDC(surface.screen) };
    if surface.dc == 0 {
        return Err(io::Error::last_os_error());
    }
    let info = BitmapInfo {
        size: 40,
        width: w as i32,
        height: -(h as i32),
        planes: 1,
        bits: 32,
        compression: 0,
        image_size: count as u32,
        x_ppm: 0,
        y_ppm: 0,
        colors: 0,
        important: 0,
    };
    let mut bits = std::ptr::null_mut();
    surface.bitmap = unsafe { CreateDIBSection(surface.screen, &info, 0, &mut bits, 0, 0) };
    if surface.bitmap == 0 || bits.is_null() {
        return Err(io::Error::last_os_error());
    }
    surface.previous = unsafe { SelectObject(surface.dc, surface.bitmap) };
    if surface.previous == 0 || surface.previous == -1 {
        return Err(io::Error::last_os_error());
    }
    unsafe {
        std::ptr::write_bytes(bits, 0, count);
    }
    identity(hwnd, Some(pid))?;
    if unsafe { PrintWindow(hwnd, surface.dc, 2) } == 0 {
        return Err(invalid("window renderer did not support capture"));
    }
    if unsafe { GdiFlush() } == 0 {
        return Err(io::Error::last_os_error());
    }
    let after = snapshot(hwnd, Some(pid))?;
    if before["width"] != after["width"]
        || before["height"] != after["height"]
        || after["minimized"] == true
    {
        return Err(invalid("window changed during capture"));
    }
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    let bgr = unsafe { std::slice::from_raw_parts(bits as *const u8, count) };
    let mut rgb = Vec::with_capacity(count / 4 * 3);
    for pixel in bgr.chunks_exact(4) {
        rgb.extend_from_slice(&[pixel[2], pixel[1], pixel[0]]);
    }
    let mut encoder = png::Encoder::new(&mut file, w as u32, h as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().map_err(io::Error::other)?;
    writer.write_image_data(&rgb).map_err(io::Error::other)?;
    writer.finish().map_err(io::Error::other)?;
    file.sync_all()?;
    Ok(())
}
pub(super) fn worker(args: &[OsString]) -> io::Result<bool> {
    if args.len() != 4
        || std::env::var_os("NSRE_CAPTURE_CHILD").as_deref() != Some(std::ffi::OsStr::new("1"))
    {
        return Err(invalid("internal capture worker"));
    }
    let path = Path::new(&args[3]);
    if std::env::var_os("NSRE_CAPTURE_BACKEND").as_deref() == Some(std::ffi::OsStr::new("wgc")) {
        let metadata = match super::wgc::capture(number(&args[1])?, number(&args[2])?, path) {
            Ok(metadata) => metadata,
            Err(error) => {
                let mut diagnostic = OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(path.with_extension("error.json"))?;
                serde_json::to_writer(
                    &mut diagnostic,
                    &json!({"backend":"wgc","error":error.to_string()}),
                )
                .map_err(io::Error::other)?;
                diagnostic.sync_all()?;
                return Err(error);
            }
        };
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path.with_extension("json"))?;
        use std::io::Write;
        serde_json::to_writer(&mut file, &metadata).map_err(io::Error::other)?;
        file.flush()?;
        file.sync_all()?;
    } else {
        write_frame(number(&args[1])?, number(&args[2])?, path)?;
    }
    Ok(true)
}
struct Temp(std::path::PathBuf);
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
pub(super) fn command(hwnd: Hwnd, pid: u32, path: &Path) -> io::Result<bool> {
    command_backend(hwnd, pid, path, false)
}
pub(super) fn command_backend(hwnd: Hwnd, pid: u32, path: &Path, wgc: bool) -> io::Result<bool> {
    let before = snapshot(hwnd, Some(pid))?;
    let parent = crate::collect::normal_dir(
        path.parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new(".")),
    )?;
    let name = path
        .file_name()
        .and_then(|s| s.to_str())
        .filter(|s| !s.contains(':') && !s.ends_with(['.', ' ']))
        .ok_or_else(|| invalid("normal capture file name required"))?;
    let output = parent.join(name);
    if output.try_exists()? {
        return Err(invalid("capture output exists"));
    }
    let temp = Temp(parent.join(format!(
            ".nsre-capture-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(io::Error::other)?
                .as_nanos()
        )));
    fs::create_dir(&temp.0)?;
    let frame = temp.0.join("frame.png");
    let mut child = Command::new(std::env::current_exe()?)
        .args(["desktop", "capture-worker"])
        .arg(hwnd.to_string())
        .arg(pid.to_string())
        .arg(&frame)
        .env("NSRE_CAPTURE_CHILD", "1")
        .env(
            "NSRE_CAPTURE_BACKEND",
            if wgc { "wgc" } else { "printwindow" },
        )
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    let start = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(s)) => break s,
            Ok(None) => {}
            Err(e) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(e);
            }
        }
        if start.elapsed() >= Duration::from_secs(5) {
            let _ = child.kill();
            let _ = child.wait();
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "capture renderer timed out; owned worker stopped",
            ));
        }
        thread::sleep(Duration::from_millis(20));
    };
    if !status.success() {
        let error_path = frame.with_extension("error.json");
        if let Ok(metadata) = fs::symlink_metadata(&error_path) {
            if metadata.is_file() && !crate::unsafe_entry(&metadata) && metadata.len() <= 8192 {
                let error: Value =
                    serde_json::from_slice(&fs::read(&error_path)?).map_err(io::Error::other)?;
                return Err(io::Error::other(format!(
                    "capture worker: {}",
                    error["error"].as_str().unwrap_or("unknown backend error")
                )));
            }
        }
        return Err(invalid(
            "window capture unsupported, protected, minimized, stale or unavailable",
        ));
    }
    let after = snapshot(hwnd, Some(pid))?;
    let (bytes, sha256) = crate::hash_file(&frame)?;
    let backend: Value = if wgc {
        serde_json::from_slice(&fs::read(frame.with_extension("json"))?)
            .map_err(io::Error::other)?
    } else {
        json!({"method":"PrintWindow fallback"})
    };
    fs::hard_link(&frame, &output)?;
    println!(
        "{}",
        json!({"schema":1,"method":backend["method"],"backend":backend,"format":"png","hwnd":hwnd.to_string(),"pid":pid,"output":output.to_string_lossy(),"bytes":bytes,"sha256":sha256,"before":before,"after":after,"content_verified":false,"live_stream":false,"minimized_supported":false,"directx_supported":"unverified","protected_content_bypassed":false})
    );
    Ok(true)
}
pub(super) fn watch_backend(
    hwnd: Hwnd,
    pid: u32,
    path: &Path,
    frames: usize,
    interval: u64,
    wgc: bool,
) -> io::Result<bool> {
    if !(1..=120).contains(&frames) || !(200..=5000).contains(&interval) {
        return Err(invalid(
            "watch requires 1..=120 frames and 200..=5000 ms interval",
        ));
    }
    let parent = crate::collect::normal_dir(
        path.parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new(".")),
    )?;
    let name = path
        .file_name()
        .and_then(|s| s.to_str())
        .filter(|s| !s.contains(':') && !s.ends_with(['.', ' ']))
        .ok_or_else(|| invalid("normal watch directory required"))?;
    let directory = parent.join(name);
    fs::create_dir(&directory)?;
    for index in 0..frames {
        if keyboard::cancelled() {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "Escape held; capture watch stopped",
            ));
        }
        {
            let _lock = keyboard::DesktopLock::acquire()?;
            command_backend(
                hwnd,
                pid,
                &directory.join(format!("frame-{index:04}.png")),
                wgc,
            )?;
        }
        if index + 1 < frames {
            for _ in 0..interval.div_ceil(50) {
                thread::sleep(Duration::from_millis(50));
                if keyboard::cancelled() {
                    return Err(io::Error::new(
                        io::ErrorKind::Interrupted,
                        "capture watch stopped",
                    ));
                }
            }
        }
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn capture_bounds() {
        assert_eq!(dimensions(2, 3).unwrap(), 24);
        for (w, h) in [(0, 1), (1, 0), (-1, 10), (8192, 8192), (9000, 1)] {
            assert!(dimensions(w, h).is_err());
        }
        assert_eq!(std::mem::size_of::<BitmapInfo>(), 40);
        assert!(worker(&[]).is_err());
    }
}
