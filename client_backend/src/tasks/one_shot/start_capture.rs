use std::{error::Error, future::Future, pin::Pin, ptr, sync::Arc};

use crate::{base::{app_data::AppData, event_loop::{EventDispatcher, EventLoop}, task::{Task, TaskMeta}}, core::task_handler::TaskHandler};

use windows::{
    Win32::Foundation::*,
    Win32::System::WinRT::Graphics::Capture::IGraphicsCaptureItemInterop,
    Win32::System::WinRT::Direct3D11::CreateDirect3D11DeviceFromDXGIDevice,
    Win32::System::WinRT::RoGetActivationFactory,
    Win32::Graphics::Direct3D11::{D3D11CreateDevice, ID3D11Device, ID3D11DeviceContext, D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_SDK_VERSION},
    Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE_HARDWARE, D3D_FEATURE_LEVEL},
    Win32::Graphics::Dxgi::{IDXGIAdapter, IDXGIDevice},
    Graphics::DirectX::DirectXPixelFormat,
    Graphics::DirectX::Direct3D11::IDirect3DDevice,
    Graphics::Capture,
    Graphics::SizeInt32,
};
use windows_core::Interface;

use windows_strings::*;

pub struct StartCaptureTask {
    meta: TaskMeta
}

impl StartCaptureTask {
    pub fn new() -> Box<StartCaptureTask> {
        let meta = TaskMeta {
            name: "capture_gameplay",
        };
        Box::new(StartCaptureTask{ meta })
    }
}

impl Task for StartCaptureTask {
    fn data(&self) -> &TaskMeta { &self.meta }
    fn execute(self: Box<Self>, app_data: Arc<AppData>, _task_handler: Arc<TaskHandler>, _event_loop: Arc<EventLoop>, _dispatcher: EventDispatcher) -> Pin<Box<dyn Future<Output = ()> + 'static>> { 
        Box::pin(start_capture(app_data))
    }
}

fn create_capture_item(hwnd: HWND) -> Result<Capture::GraphicsCaptureItem, Box<dyn Error>> {
    unsafe {
        // obtain interop factory
        let activation_class = HSTRING::from("Windows.Graphics.Capture.GraphicsCaptureItem");
        let factory = match RoGetActivationFactory::<IGraphicsCaptureItemInterop> (&activation_class){
            Ok(factory) => factory,
            Err(e) => {
                let err = format!("start capture failed: RoGetActivationFactory failed: {:?}", e);
                return Err(err.into());
            }
        };

        // create graphics capture item
        let capture_item: Capture::GraphicsCaptureItem = match factory.CreateForWindow(hwnd){
            Ok(capture_item) => capture_item,
            Err(e) => {
                let err = format!("start capture failed: IGraphicsCaptureItemInterop::CreateForWindow failed: {:?}", e);
                return Err(err.into());
            }
        
        };

        return Ok(capture_item);
    }

}

fn create_d3d11_device() -> Result<(ID3D11Device, IDirect3DDevice, ID3D11DeviceContext), Box<dyn Error>> {
    unsafe {
        let mut device: Option<ID3D11Device> = None;
        let mut context: Option<ID3D11DeviceContext> = None;
        let mut feature_level: D3D_FEATURE_LEVEL = D3D_FEATURE_LEVEL(0);

        D3D11CreateDevice(
            None::<&IDXGIAdapter>, // Default adapter
            D3D_DRIVER_TYPE_HARDWARE,     // Use the GPU
            HMODULE(ptr::null_mut()),              // No software rasterizer
            D3D11_CREATE_DEVICE_BGRA_SUPPORT, // BGRA support for Direct2D interop
            None,                         // Use default feature levels
            D3D11_SDK_VERSION,            // SDK version
            Some(&mut device),            // Output device
            Some(&mut feature_level),     // Output feature level
            Some(&mut context),           // Output immediate context
        )?;

        let d3d11_device = device.expect("Device is not available");

        // convert ID3D11Device into IDXGIDevice
        let dxgi_device: IDXGIDevice = d3d11_device.cast()?; // Get DXGI Device
        // convert IInspectable into IDirect3DDevice
        let direct3d_device: IDirect3DDevice = CreateDirect3D11DeviceFromDXGIDevice(&dxgi_device)?
            .cast()?;

        Ok((d3d11_device, direct3d_device, context.unwrap()))
    }
}
fn create_frame_pool(d3d_device: &IDirect3DDevice, size: SizeInt32) -> Result<Capture::Direct3D11CaptureFramePool, Box<dyn Error>> {
    // Create the frame pool using the associated function
    Ok(Capture::Direct3D11CaptureFramePool::Create(
        d3d_device,                      // Direct3D device
        DirectXPixelFormat::B8G8R8A8UIntNormalized, // Pixel format
        2,                               // Number of buffers
        size,                            // Size of the buffers
    )?)
}


async fn start_capture(app_data: Arc<AppData>) {
    unsafe {
        // check if game_hwnd is available
        let game_hwnd = match app_data.get_game_hwnd().await{
            Some(hwnd) => hwnd,
            None => {
                log::info!("start capture failed: Game window is not available");
                return;
            }
        };

        // create capture item
        let capture_item = match create_capture_item(game_hwnd){
            Ok(capture_item) => capture_item,
            Err(e) => {
                log::info!("start capture failed: create_capture_item failed: {:?}", e);
                return;
            }
        };

        // create 3d device
        let (d3d11_device, id3d_device, d3d_context) = match create_d3d11_device(){
            Ok((device11, d3d_device, context)) => (device11, d3d_device, context),
            Err(e) => {
                log::info!("start capture failed: create_d3d11_device failed: {:?}", e);
                return;
            }
        };

        // create frame pool
        let frame_size = capture_item.Size().expect("Size is not available");
        let frame_pool = match create_frame_pool(&id3d_device, frame_size){
            Ok(frame_pool) => frame_pool,
            Err(e) => {
                log::info!("start capture failed: create_frame_pool failed: {:?}", e);
                return;
            }
        }; 

        let session: Capture::GraphicsCaptureSession = match frame_pool.CreateCaptureSession(&capture_item){
            Ok(session) => session,
            Err(e) => {
                log::info!("start capture failed: CreateCaptureSession failed: {:?}", e);
                return;
            }
        };

        match session.StartCapture(){
            Ok(_) => { log::info!("capture successfully started"); },
            Err(e) => {
                log::info!("start capture failed: StartCapture failed: {:?}", e);
                return;
            }
        }

        // set session in app_data
        AppData::set_capture_session(app_data, Some(session)).await;
    }
}

