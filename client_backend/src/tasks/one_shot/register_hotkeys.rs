use std::{future::Future, pin::Pin, ptr, sync::Arc};
use windows::{
    Win32::Foundation::*,
    Win32::UI::Input::KeyboardAndMouse::*,
};

use crate::{base::{app_data::AppData, event_loop::{EventDispatcher, EventLoop}, task::{Task, TaskMeta}}, core::task_handler::TaskHandler};

pub struct RegisterHotkeysTask {
    meta: TaskMeta
}

impl RegisterHotkeysTask {
    pub fn new() -> Box<RegisterHotkeysTask> {
        let meta = TaskMeta{
            name: "register_hotkeys",
        };
        Box::new(RegisterHotkeysTask{ meta })
    }
}

impl Task for RegisterHotkeysTask{
    fn data(&self) -> &TaskMeta { &self.meta }
    fn execute(self: Box<Self>, app_data: Arc<AppData>,  _task_handler: Arc<TaskHandler>, _event_loop: Arc<EventLoop>, _dispatcher: EventDispatcher) -> Pin<Box<dyn Future<Output = ()> + 'static>> { 
        Box::pin(register_hotkeys())
    }
}

/// register_hotkeys
/// will be running headless, meaning no window handle supplied
/// This also means that the only way to recieve input is to poll
/// the message queue and check if a hotkey was pressed
async fn register_hotkeys() {
    log::info!("Registering hotkeys");
    unsafe {
        let _ = RegisterHotKey(
            Some(HWND(ptr::null_mut())),
            1,
            MOD_ALT,
            'Q' as u32,
        );
        log::info!("Registered Quit Hotkey: ALT + Q");

        let _ = RegisterHotKey(
            Some(HWND(ptr::null_mut())),
            2,
            MOD_ALT,
            'R' as u32,
        );
        log::info!("Registered Start Capture Hotkey(2): ALT + R");

        let _ = RegisterHotKey(
            Some(HWND(ptr::null_mut())),
            3,
            MOD_ALT,
            'E' as u32,
        );
        log::info!("Registered Stop Capture Hotkey(3): ALT + E");

        let _ = RegisterHotKey(
            Some(HWND(ptr::null_mut())),
            4,
            MOD_ALT,
            'P' as u32,
        );
        log::info!("Registered Print Windows Hotkey: ALT + P");
    };
}
