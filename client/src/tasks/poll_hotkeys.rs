use std::{future::Future, pin::Pin, ptr, sync::Arc, time::Duration};
use tokio::time;
use windows::{
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
};

use crate::{base::{event::{Event, HotkeyEventData}, event_loop::{EventDispatcher, EventLoop}, task::{Task, TaskMeta}}, core::task_handler::TaskHandler};

pub struct PollHotkeysTask {
    meta: TaskMeta
}

impl PollHotkeysTask {
    pub fn new() -> Box<PollHotkeysTask> {
        let meta = TaskMeta{
            name: "poll_hotkeys",
        };
        Box::new(PollHotkeysTask { meta })
    }
}

impl Task for PollHotkeysTask{
    fn data(&self) -> &TaskMeta { &self.meta }
    fn execute(self: Box<Self>, _task_handler: Arc<TaskHandler>, _event_loop: Arc<EventLoop>, dispatcher: EventDispatcher) -> Pin<Box<dyn Future<Output = ()> + 'static>> { 
        Box::pin(poll_hotkeys(dispatcher))
    }
}


pub async fn poll_hotkeys(dispatcher: EventDispatcher){
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
                    let new_event = Event::HotKeyEvent(HotkeyEventData{
                        id: msg.wParam.0 as u32,
                        vks: msg.lParam.0 as u32,
                    });
                    dispatcher.dispatch(new_event).await;
                }
            }
        }
        time::sleep(Duration::from_millis(50)).await;
    }
}
