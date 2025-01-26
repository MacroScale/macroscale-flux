use std::{future::Future, pin::Pin, sync::Arc};

use crate::core::task_handler::TaskHandler;

use super::event_loop::{EventDispatcher, EventLoop};

#[derive(Clone)]
pub struct TaskMeta {
    pub name: &'static str,
}

pub trait Task: Send {
   fn data(&self) -> &TaskMeta;
   fn execute(self: Box<Self>, task_handler: Arc<TaskHandler>, event_loop: Arc<EventLoop>, dispatcher: EventDispatcher) -> Pin<Box<dyn Future<Output = ()> + 'static>>;
}
