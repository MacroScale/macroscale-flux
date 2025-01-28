
use windows::{
    Win32::Foundation::*,
    Graphics::Capture::*,
};

pub struct CaptureBuffer {
    frame_pool: Direct3D11CaptureFramePool,
    session: GraphicsCaptureSession,
}

impl CaptureBuffer{

    pub fn new(hwnd: Option<HWND>) -> Option<Self> {

        // check if hwnd is none
        if hwnd.is_none() { return None; }

        // Step 1: Create a Direct3D device
        let d3d_device = match create_d3d_device(){
            Ok(device) => device,
            Err(e) => {
                log::error!("Error creating Direct3D device: {}", e);
                return None;
            }
        };

        // Step 2: Create a Direct3D11 device for Windows Graphics Capture
        let d3d11_device = match create_direct3d_device(d3d_device.clone()){
            Ok(device) => device,
            Err(e) => {
                log::error!("Error creating Direct3D11 device: {}", e);
                return None;
            }
        };

        // Step 3: Create GraphicsCaptureItem for the specified HWND
        let item = unsafe { 
            match GraphicsCaptureItem::CreateForWindow(hwnd){
                Ok(item) => item,
                Err(e) => {
                    log::error!("Error creating GraphicsCaptureItem: {}", e);
                    return None;
                }
            } 
        };

        // Step 4: Set up the frame pool
        let size = item.Size();
        let frame_pool = match Direct3D11CaptureFramePool::Create( d3d11_device, DirectXPixelFormat::B8G8R8A8UIntNormalized, 2, size) {
            Ok(pool) => pool,
            Err(e) => {
                log::error!("Error creating Direct3D11CaptureFramePool: {}", e);
                return None;
            }
        };

        // Step 5: Create the capture session
        let session = match frame_pool.CreateCaptureSession(item){
            Ok(session) => session,
            Err(e) => {
                log::error!("Error creating GraphicsCaptureSession: {}", e);
                return None;
            }
        };

        Some(Self { frame_pool, session })
    }

}
/// Create a Direct3D11 device
fn create_d3d_device() -> Result<ID3D11Device> {
    let mut device: Option<ID3D11Device> = None;

    unsafe {
fn D3D11CreateDevice<P0, P1>(
    padapter: P0,
    drivertype: D3D_DRIVER_TYPE,
    software: P1,
    flags: D3D11_CREATE_DEVICE_FLAG,
    pfeaturelevels: Option<&[D3D_FEATURE_LEVEL]>,
    sdkversion: u32,
    ppdevice: Option<*mut Option<ID3D11Device>>,
    pfeaturelevel: Option<*mut D3D_FEATURE_LEVEL>,
    ppimmediatecontext: Option<*mut Option<ID3D11DeviceContext>>,
) -> Result<()>
where
    P0: Param<IDXGIAdapter>,
    P1: Param<HMODULE>,

        D3D11CreateDevice(
            None,
            D3D_DRIVER_TYPE_HARDWARE,
            None,
            D3D11_CREATE_DEVICE_BGRA_SUPPORT,
            None,
            0,
            D3D11_SDK_VERSION,
            &mut device,
            None,
            None,
        )?;
    }

    Ok(device.unwrap())
}

/// Create a Direct3D device for the Windows Graphics Capture API
fn create_direct3d_device(d3d_device: ID3D11Device) -> Result<IDirect3DDevice> {
    let dxgi_device: IDXGIDevice = d3d_device.cast()?;
    let mut d3d11_device: Option<IDirect3DDevice> = None;

    unsafe {
        CreateDirect3D11DeviceFromDXGIDevice(dxgi_device, &mut d3d11_device)?;
    }

    Ok(d3d11_device.unwrap())
}
