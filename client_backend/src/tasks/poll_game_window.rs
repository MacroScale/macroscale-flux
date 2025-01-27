use std::{future::Future, pin::Pin, ptr, sync::Arc, time::Duration};
use tokio::time;
use windows::{
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
};

use crate::{base::{app_data::AppData, event::{Event, HotkeyEventData}, event_loop::{EventDispatcher, EventLoop}, task::{Task, TaskMeta}}, core::task_handler::TaskHandler, utils};

pub struct PollGameWindowTask {
    meta: TaskMeta
}

impl PollGameWindowTask {
    pub fn new() -> Box<PollGameWindowTask> {
        let meta = TaskMeta{
            name: "poll_game_window",
        };
        Box::new(PollGameWindowTask { meta })
    }
}

impl Task for PollGameWindowTask{
    fn data(&self) -> &TaskMeta { &self.meta }
    fn execute(self: Box<Self>, app_data: Arc<AppData>,  _task_handler: Arc<TaskHandler>, _event_loop: Arc<EventLoop>, dispatcher: EventDispatcher) -> Pin<Box<dyn Future<Output = ()> + 'static>> { 
        Box::pin(poll_game_window(app_data.clone(), dispatcher))
    }
}


pub async fn poll_game_window(app_data: Arc<AppData>, dispatcher: EventDispatcher){
    loop {
        unsafe {
            if app_data.get_current_hwnd().await.is_none() {continue;}

        }
        time::sleep(Duration::from_millis(50)).await;
    }
}
