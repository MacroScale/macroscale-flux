use std::{future::Future, pin::Pin, ptr};
use tokio::sync::mpsc::Sender;
use windows::{
    Win32::Foundation::*,
    Win32::UI::Input::KeyboardAndMouse::*,
};

use crate::base::{event::Event, event_loop::EventDispatcher, task::{Task, TaskMeta}};

pub struct RegisterHotkeysTask {
    meta: TaskMeta
}

impl RegisterHotkeysTask {
    pub fn new() -> Box<RegisterHotkeysTask> {
        let meta = TaskMeta{
            id: 0,
            name: "register_hotkeys",
        };
        Box::new(RegisterHotkeysTask{ meta })
    }
}

impl Task for RegisterHotkeysTask{
    fn data(&self) -> &TaskMeta { &self.meta }
    fn execute(self: Box<Self>, dispatcher: EventDispatcher) -> Pin<Box<dyn Future<Output = ()> + 'static>> { 
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
            69,
            MOD_ALT,
            'Q' as u32,
        );
        log::info!("Registered Quit Hotkey: ALT + Q");

        let _ = RegisterHotKey(
            Some(HWND(ptr::null_mut())),
            420,
            MOD_ALT,
            'W' as u32,
        );
        log::info!("Registered Capture Hotkey: ALT + W")
    };
}
