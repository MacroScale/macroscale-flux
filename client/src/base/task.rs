use std::{cell::RefCell, future::Future, pin::Pin, rc::Rc, sync::Arc};

use tokio::sync::{mpsc::Sender, Mutex};

use super::{event::Event, event_loop::{EventDispatcher, EventLoop}};

#[derive(Clone)]
pub struct TaskMeta {
    pub id: u8,
    pub name: &'static str,
}

pub trait Task: Send {
   fn data(&self) -> &TaskMeta;
   fn execute(self: Box<Self>, event_loop: Arc<EventLoop>, dispatcher: EventDispatcher) -> Pin<Box<dyn Future<Output = ()> + 'static>>;
}
