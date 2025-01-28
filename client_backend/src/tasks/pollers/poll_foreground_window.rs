use std::{future::Future, pin::Pin, ptr, sync::Arc, time::Duration};
use tokio::time;
use windows::{
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
};

use crate::{base::{app_data::AppData, event::{ChangeForegroundWindowData, Event, EventType}, event_loop::{EventDispatcher, EventLoop}, task::{Task, TaskMeta}}, core::task_handler::TaskHandler, utils};

pub struct PollForegroundWindowTask {
    meta: TaskMeta
}

impl PollForegroundWindowTask {
    pub fn new() -> Box<PollForegroundWindowTask> {
        let meta = TaskMeta{
            name: "poll_foreground_window",
        };
        Box::new(PollForegroundWindowTask { meta })
    }
}

impl Task for PollForegroundWindowTask{
    fn data(&self) -> &TaskMeta { &self.meta }
    fn execute(self: Box<Self>, app_data: Arc<AppData>,  _task_handler: Arc<TaskHandler>, _event_loop: Arc<EventLoop>, dispatcher: EventDispatcher) -> Pin<Box<dyn Future<Output = ()> + 'static>> { 
        Box::pin(poll_foreground(app_data.clone(), dispatcher))
    }
}


pub async fn poll_foreground(app_data: Arc<AppData>, dispatcher: EventDispatcher){
    loop {
        time::sleep(Duration::from_millis(100)).await;
        unsafe {
            let previous_hwnd_op = app_data.get_current_hwnd().await;
            let foreground_window_op = utils::get_foreground_window_hwnd();

            if previous_hwnd_op != foreground_window_op {
                // send event to dispatcher
                let event_data = ChangeForegroundWindowData{hwnd: foreground_window_op}; 
                let event = Event(EventType::ChangeForegroundProcessHWND(event_data));
                dispatcher.dispatch(event).await;
            }
        }
    }
}
