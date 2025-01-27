use std::{future::Future, pin::Pin, sync::Arc};

use crate::{base::{app_data::AppData, event_loop::{EventDispatcher, EventLoop}, task::{Task, TaskMeta}}, core::task_handler::TaskHandler};

pub struct CaptureGameplayTask {
    meta: TaskMeta
}

impl CaptureGameplayTask {
    pub fn new() -> Box<CaptureGameplayTask> {
        let meta = TaskMeta {
            name: "capture_gameplay",
        };
        Box::new(CaptureGameplayTask{ meta })
    }
}

impl Task for CaptureGameplayTask {
    fn data(&self) -> &TaskMeta { &self.meta }
    fn execute(self: Box<Self>, app_data: Arc<AppData>, _task_handler: Arc<TaskHandler>, _event_loop: Arc<EventLoop>, _dispatcher: EventDispatcher) -> Pin<Box<dyn Future<Output = ()> + 'static>> { 
        Box::pin(action_capture())
    }
}

async fn action_capture() {
}
