use std::{future::Future, pin::Pin, sync::Arc};

use crate::{base::{app_data::AppData, event_loop::{EventDispatcher, EventLoop}, task::{Task, TaskMeta}}, core::task_handler::TaskHandler};

pub struct SaveSessionTovideo {
    meta: TaskMeta
}

impl SaveSessionTovideo {
    pub fn new() -> Box<SaveSessionTovideo> {
        let meta = TaskMeta {
            name: "save_session_to_video",
        };
        Box::new(SaveSessionTovideo{ meta })
    }
}

impl Task for SaveSessionTovideo {
    fn data(&self) -> &TaskMeta { &self.meta }
    fn execute(self: Box<Self>, app_data: Arc<AppData>, _task_handler: Arc<TaskHandler>, _event_loop: Arc<EventLoop>, dispatcher: EventDispatcher) -> Pin<Box<dyn Future<Output = ()> + 'static>> { 
        Box::pin(save_session_to_video(app_data))
    }
}

async fn save_session_to_video(app_data: Arc<AppData>) {

}
