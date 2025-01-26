use std::{future::Future, pin::Pin, ptr, time::Duration};
use tokio::{sync::mpsc::Sender, time};
use windows::{
    Win32::Foundation::*,
    Win32::UI::Input::KeyboardAndMouse::*,
};

use crate::base::{event::Event, event_loop::EventDispatcher, task::{Task, TaskMeta}};

pub struct HotkeyHandler{
    meta: TaskMeta
}

impl HotkeyHandler {
    pub fn new() -> Box<HotkeyHandler> {
        let meta = TaskMeta{
            id: 0,
            name: "register_hotkeys",
        };
        Box::new(HotkeyHandler{ meta })
    }
}

impl Task for HotkeyHandler {
    fn data(&self) -> &TaskMeta { &self.meta }
    fn execute(self: Box<Self>, dispatcher: EventDispatcher) -> Pin<Box<dyn Future<Output = ()> + 'static>> { 
        Box::pin(handle_hotkey_input())
    }
}

async fn handle_hotkey_input() {

    loop{
        time::sleep(Duration::from_millis(50)).await;
    }

}
