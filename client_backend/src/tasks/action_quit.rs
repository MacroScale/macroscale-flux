use std::{future::Future, pin::Pin, sync::Arc};
use crate::{base::{app_data::AppData, event_loop::{EventDispatcher, EventLoop}, task::{Task, TaskMeta}}, core::task_handler::TaskHandler};

pub struct ActionQuitTask {
    meta: TaskMeta
}

impl ActionQuitTask {
    pub fn new() -> Box<ActionQuitTask> {
        let meta = TaskMeta{
            name: "action_quit",
        };
        Box::new(ActionQuitTask{ meta })
    }
}

impl Task for ActionQuitTask {
    fn data(&self) -> &TaskMeta { &self.meta }
    fn execute(self: Box<Self>, app_data: Arc<AppData>,  _task_handler: Arc<TaskHandler>, _event_loop: Arc<EventLoop>, _dispatcher: EventDispatcher) -> Pin<Box<dyn Future<Output = ()> + 'static>> { 
        Box::pin(action_quit())
    }
}

async fn action_quit() {
    std::process::exit(0);
}
