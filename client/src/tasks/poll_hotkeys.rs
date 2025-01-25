use std::{future::Future, pin::Pin, ptr, time::Duration};
use tokio::time;
use windows::{
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
};

use crate::base::task::{Task, TaskMeta};

pub struct PollHotkeysTask {
    meta: TaskMeta
}

impl PollHotkeysTask {
    pub fn new() -> Box<PollHotkeysTask> {
        let meta = TaskMeta{
            id: 0,
            name: "poll_hotkeys",
        };
        Box::new(PollHotkeysTask { meta })
    }
}

impl Task for PollHotkeysTask{
    fn data(&self) -> &TaskMeta { &self.meta }
    fn execute(self: Box<Self>) -> Pin<Box<dyn Future<Output = ()> + 'static>> { 
        Box::pin(poll_hotkeys())
    }
}


pub async fn poll_hotkeys(){
    loop {
        unsafe {
            let mut msg = MSG {
                hwnd: HWND(ptr::null_mut()),
                message: 0,
                wParam: WPARAM(0),
                lParam: LPARAM(0),
                time: 0,
                pt: std::mem::zeroed(),
            };

            let peek_value = PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool();
            if  peek_value {
                if msg.message == WM_HOTKEY {
                    // figure out what hotkey was pressed
                    log::info!("message: {:?}", msg);
                }
            }
        }
        time::sleep(Duration::from_millis(50)).await;
    }
}
