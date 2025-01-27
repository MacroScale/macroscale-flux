use std::{future::Future, pin::Pin, sync::Arc};
use crate::{base::{app_data::AppData, event_loop::{EventDispatcher, EventLoop}, task::{Task, TaskMeta}}, core::task_handler::TaskHandler};

pub struct QuitApplicationTask {
    meta: TaskMeta
}

impl QuitApplicationTask {
    pub fn new() -> Box<QuitApplicationTask> {
        let meta = TaskMeta{
            name: "quit_application",
        };
        Box::new(QuitApplicationTask{ meta })
    }
}

impl Task for QuitApplicationTask {
    fn data(&self) -> &TaskMeta { &self.meta }
    fn execute(self: Box<Self>, _app_data: Arc<AppData>,  _task_handler: Arc<TaskHandler>, _event_loop: Arc<EventLoop>, _dispatcher: EventDispatcher) -> Pin<Box<dyn Future<Output = ()> + 'static>> { 
        Box::pin(action_quit())
    }
}

async fn action_quit() {
    std::process::exit(0);
}
