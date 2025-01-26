use std::{future::Future, pin::Pin};

use tokio::sync::mpsc::Sender;

use super::{event::Event, event_loop::EventDispatcher};

#[derive(Clone)]
pub struct TaskMeta {
    pub id: u8,
    pub name: &'static str,
}

pub trait Task: Send {
   fn data(&self) -> &TaskMeta;
   fn execute(self: Box<Self>, dispatcher: EventDispatcher) -> Pin<Box<dyn Future<Output = ()> + 'static>>;
}
