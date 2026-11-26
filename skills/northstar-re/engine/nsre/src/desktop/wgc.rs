//! Windows Graphics Capture of an explicitly selected HWND, using an owned D3D11 device.
use super::*;
use std::{
    fs::OpenOptions,
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Instant,
};
use windows::{
    core::{factory, IInspectable, Interface},
    Foundation::TypedEventHandler,
    Graphics::{
        Capture::{Direct3D11CaptureFramePool, GraphicsCaptureItem, GraphicsCaptureSession},
        DirectX::{Direct3D11::IDirect3DDevice, DirectXPixelFormat},
    },
    Win32::{
        Foundation::{HMODULE, HWND},
        Graphics::{
            Direct3D::{D3D_DRIVER_TYPE_HARDWARE, D3D_DRIVER_TYPE_WARP},
            Direct3D11::*,
            Dxgi::IDXGIDevice,
        },
        System::WinRT::{
            Direct3D11::{CreateDirect3D11DeviceFromDXGIDevice, IDirect3DDxgiInterfaceAccess},
            Graphics::Capture::IGraphicsCaptureItemInterop,
            RoInitialize, RoUninitialize, RO_INIT_MULTITHREADED,
        },
    },
};
fn failure(e: windows::core::Error) -> io::Error {
    io::Error::other(format!("{} (HRESULT 0x{:08x})", e, e.code().0 as u32))
}
fn stage(label: &'static str) -> impl FnOnce(windows::core::Error) -> io::Error {
    move |e| {
        io::Error::other(format!(
            "{label}: {} (HRESULT 0x{:08x})",
            e,
            e.code().0 as u32
        ))
    }
}
struct Runtime;
impl Runtime {
    fn new() -> io::Result<Self> {
        unsafe { RoInitialize(RO_INIT_MULTITHREADED) }.map_err(failure)?;
        Ok(Self)
    }
}
impl Drop for Runtime {
    fn drop(&mut self) {
        unsafe { RoUninitialize() }
    }
}
struct Session {
    session: GraphicsCaptureSession,
    pool: Direct3D11CaptureFramePool,
    arrived_token: i64,
}
impl Drop for Session {
    fn drop(&mut self) {
        let _ = self.session.Close();
        let _ = self.pool.RemoveFrameArrived(self.arrived_token);
        let _ = self.pool.Close();
    }
}
struct Mapping<'a> {
    context: &'a ID3D11DeviceContext,
    texture: &'a ID3D11Texture2D,
}
impl Drop for Mapping<'_> {
    fn drop(&mut self) {
        unsafe { self.context.Unmap(self.texture, 0) }
    }
}
struct Progress(std::fs::File);
impl Progress {
    fn new(path: &Path) -> io::Result<Self> {
        Ok(Self(
            OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(path.with_extension("progress.json"))?,
        ))
    }
    fn mark(&mut self, phase: &str) -> io::Result<()> {
        use std::io::{Seek, SeekFrom, Write};
        self.0.seek(SeekFrom::Start(0))?;
        self.0.set_len(0)?;
        serde_json::to_writer(&mut self.0, &json!({"phase":phase})).map_err(io::Error::other)?;
        self.0.flush()?;
        self.0.sync_all()
    }
}
pub(super) fn capture(hwnd: Hwnd, pid: u32, path: &Path) -> io::Result<Value> {
    let mut progress = Progress::new(path)?;
    progress.mark("initialize WinRT")?;
    let _runtime = Runtime::new()?;
    let before = snapshot(hwnd, Some(pid))?;
    if before["minimized"] == true {
        return Err(invalid("restore minimized window before WGC capture"));
    }
    if !GraphicsCaptureSession::IsSupported().map_err(failure)? {
        return Err(invalid(
            "Windows Graphics Capture unavailable in this session",
        ));
    }
    progress.mark("capture item factory")?;
    let interop: IGraphicsCaptureItemInterop =
        factory::<GraphicsCaptureItem, IGraphicsCaptureItemInterop>()
            .map_err(stage("capture item factory"))?;
    progress.mark("create item for selected HWND")?;
    let item: GraphicsCaptureItem =
        unsafe { interop.CreateForWindow(HWND(hwnd as *mut std::ffi::c_void)) }
            .map_err(stage("CreateForWindow"))?;
    let size = item.Size().map_err(failure)?;
    super::capture::dimensions(size.Width as i64, size.Height as i64)?;
    progress.mark("create D3D11 device")?;
    let mut device = None;
    let mut context = None;
    let hardware = unsafe {
        D3D11CreateDevice(
            None,
            D3D_DRIVER_TYPE_HARDWARE,
            HMODULE::default(),
            D3D11_CREATE_DEVICE_BGRA_SUPPORT,
            None,
            D3D11_SDK_VERSION,
            Some(&mut device),
            None,
            Some(&mut context),
        )
    }
    .is_ok();
    if !hardware {
        unsafe {
            D3D11CreateDevice(
                None,
                D3D_DRIVER_TYPE_WARP,
                HMODULE::default(),
                D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                None,
                D3D11_SDK_VERSION,
                Some(&mut device),
                None,
                Some(&mut context),
            )
        }
        .map_err(failure)?;
    }
    let device = device.ok_or_else(|| invalid("D3D11 device missing"))?;
    let context = context.ok_or_else(|| invalid("D3D11 context missing"))?;
    let dxgi: IDXGIDevice = device.cast().map_err(failure)?;
    let native: IDirect3DDevice = unsafe { CreateDirect3D11DeviceFromDXGIDevice(&dxgi) }
        .map_err(failure)?
        .cast()
        .map_err(failure)?;
    progress.mark("create free threaded frame pool")?;
    let pool = Direct3D11CaptureFramePool::CreateFreeThreaded(
        &native,
        DirectXPixelFormat::B8G8R8A8UIntNormalized,
        2,
        size,
    )
    .map_err(failure)?;
    let arrived = Arc::new(AtomicBool::new(false));
    let arrived_signal = Arc::clone(&arrived);
    let arrived_token = pool
        .FrameArrived(
            &TypedEventHandler::<Direct3D11CaptureFramePool, IInspectable>::new(move |_, _| {
                arrived_signal.store(true, Ordering::Release);
                Ok(())
            }),
        )
        .map_err(stage("subscribe FrameArrived"))?;
    let session = pool
        .CreateCaptureSession(&item)
        .map_err(stage("CreateCaptureSession"))?;
    let session = Session {
        session,
        pool,
        arrived_token,
    };
    progress.mark("start capture")?;
    session
        .session
        .StartCapture()
        .map_err(stage("StartCapture"))?;
    let start = Instant::now();
    progress.mark("wait for FrameArrived")?;
    let frame = loop {
        identity(hwnd, Some(pid))?;
        if keyboard::cancelled() {
            return Err(invalid("WGC capture cancelled"));
        }
        if !arrived.load(Ordering::Acquire) {
            if start.elapsed() >= Duration::from_millis(2000) {
                return Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    "no WGC FrameArrived event",
                ));
            }
            thread::sleep(Duration::from_millis(10));
            continue;
        }
        match session.pool.TryGetNextFrame() {
            Ok(frame) => break frame,
            Err(_) if start.elapsed() < Duration::from_millis(2000) => {
                thread::sleep(Duration::from_millis(10))
            }
            Err(e) => return Err(stage("first frame wait")(e)),
        }
    };
    progress.mark("read capture frame surface")?;
    let content = frame.ContentSize().map_err(failure)?;
    super::capture::dimensions(content.Width as i64, content.Height as i64)?;
    let surface = frame.Surface().map_err(failure)?;
    let access: IDirect3DDxgiInterfaceAccess = surface.cast().map_err(failure)?;
    let texture: ID3D11Texture2D = unsafe { access.GetInterface() }.map_err(failure)?;
    let mut desc = D3D11_TEXTURE2D_DESC::default();
    unsafe { texture.GetDesc(&mut desc) };
    if content.Width as u32 > desc.Width
        || content.Height as u32 > desc.Height
        || desc.Format != windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM
        || desc.SampleDesc.Count != 1
        || desc.MipLevels != 1
        || desc.ArraySize != 1
    {
        return Err(invalid("WGC content size exceeds texture"));
    }
    super::capture::dimensions(desc.Width as i64, desc.Height as i64)?;
    desc.Usage = D3D11_USAGE_STAGING;
    desc.BindFlags = 0;
    desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ.0 as u32;
    desc.MiscFlags = 0;
    progress.mark("create staging texture")?;
    let mut staging = None;
    unsafe { device.CreateTexture2D(&desc, None, Some(&mut staging)) }.map_err(failure)?;
    let staging = staging.ok_or_else(|| invalid("missing staging texture"))?;
    unsafe { context.CopyResource(&staging, &texture) };
    progress.mark("map staging texture")?;
    let mut map = D3D11_MAPPED_SUBRESOURCE::default();
    unsafe { context.Map(&staging, 0, D3D11_MAP_READ, 0, Some(&mut map)) }
        .map_err(stage("staging map"))?;
    let mapped = Mapping {
        context: &context,
        texture: &staging,
    };
    let width = content.Width as usize;
    let height = content.Height as usize;
    let pitch = map.RowPitch as usize;
    if map.pData.is_null()
        || pitch < width * 4
        || pitch
            .checked_mul(height)
            .is_none_or(|n| n > 256 * 1024 * 1024)
    {
        return Err(invalid("invalid mapped capture pitch"));
    }
    let bytes = unsafe { std::slice::from_raw_parts(map.pData as *const u8, pitch * height) };
    let mut rgb = Vec::with_capacity(width * height * 3);
    for row in 0..height {
        for pixel in bytes[row * pitch..row * pitch + width * 4].chunks_exact(4) {
            rgb.extend_from_slice(&[pixel[2], pixel[1], pixel[0]]);
        }
    }
    drop(mapped);
    let timestamp = frame.SystemRelativeTime().map_err(failure)?.Duration;
    frame.Close().map_err(failure)?;
    let after = snapshot(hwnd, Some(pid))?;
    if after["minimized"] == true
        || before["width"] != after["width"]
        || before["height"] != after["height"]
    {
        return Err(invalid("window changed during WGC capture"));
    }
    progress.mark("encode PNG")?;
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    let mut encoder = png::Encoder::new(&mut file, width as u32, height as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().map_err(io::Error::other)?;
    writer.write_image_data(&rgb).map_err(io::Error::other)?;
    writer.finish().map_err(io::Error::other)?;
    file.sync_all()?;
    Ok(
        json!({"method":"Windows Graphics Capture","width":width,"height":height,"frame_timestamp_100ns":timestamp,"hardware_device":hardware,"content_verified":false,"protected_content_bypassed":false}),
    )
}

pub(super) fn capabilities() -> Value {
    let supported = (|| -> io::Result<bool> {
        let _runtime = Runtime::new()?;
        GraphicsCaptureSession::IsSupported().map_err(failure)
    })();
    match supported {
        Ok(value) => {
            json!({"schema":1,"wgc_supported":value,"printwindow_fallback_available":true,"keyboard_available":true,"uia_available":true,"live_video_stream":false,"windows11_acceptance":"unverified","freepi_acceptance":"unverified"})
        }
        Err(e) => {
            json!({"schema":1,"wgc_supported":false,"reason":e.to_string(),"printwindow_fallback_available":true})
        }
    }
}
