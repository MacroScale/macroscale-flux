use std::{future::Future, pin::Pin, sync::Arc};

use crate::{base::{app_data::AppData, event_loop::{EventDispatcher, EventLoop}, task::{Task, TaskMeta}}, core::task_handler::TaskHandler};

pub struct ActionCaptureTask {
    meta: TaskMeta
}

impl ActionCaptureTask {
    pub fn new() -> Box<ActionCaptureTask> {
        let meta = TaskMeta {
            name: "action_capture",
        };
        Box::new(ActionCaptureTask{ meta })
    }
}

impl Task for ActionCaptureTask {
    fn data(&self) -> &TaskMeta { &self.meta }
    fn execute(self: Box<Self>, app_data: Arc<AppData>, _task_handler: Arc<TaskHandler>, _event_loop: Arc<EventLoop>, _dispatcher: EventDispatcher) -> Pin<Box<dyn Future<Output = ()> + 'static>> { 
        Box::pin(action_capture())
    }
}

async fn action_capture() {
}
