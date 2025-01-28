use std::{future::Future, pin::Pin, sync::Arc, time::Duration};
use tokio::time;

use windows::{
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
};

use crate::{base::{app_data::AppData, event_loop::{EventDispatcher, EventLoop}, task::{Task, TaskMeta}}, core::task_handler::TaskHandler};

pub struct WindowCapturerTask {
    meta: TaskMeta
}

impl WindowCapturerTask {
    pub fn new() -> Box<WindowCapturerTask> {
        let meta = TaskMeta{
            name: "window_capturer",
        };
        Box::new(WindowCapturerTask{ meta })
    }
}

impl Task for WindowCapturerTask{
    fn data(&self) -> &TaskMeta { &self.meta }
    fn execute(self: Box<Self>, app_data: Arc<AppData>,  task_handler: Arc<TaskHandler>, event_loop: Arc<EventLoop>, dispatcher: EventDispatcher) -> Pin<Box<dyn Future<Output = ()> + 'static>> { 
        Box::pin(window_capture(task_handler.clone(), event_loop.clone(), dispatcher.clone()))
    }
}


async fn window_capture(task_handler: Arc<TaskHandler>, event_loop: Arc<EventLoop>, dispatcher: EventDispatcher){
    loop {
        time::sleep(Duration::from_millis(50)).await;
    }
}
