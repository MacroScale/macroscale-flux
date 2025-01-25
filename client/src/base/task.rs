use std::{future::Future, pin::Pin};

use tokio::sync::mpsc::Sender;

use super::event::Event;

#[derive(Clone)]
pub struct TaskMeta {
    pub id: u8,
    pub name: &'static str,
}

pub trait Task: Send {
   fn data(&self) -> &TaskMeta;
   fn execute(self: Box<Self>, event_dispatch_channel: Sender<Event>) -> Pin<Box<dyn Future<Output = ()> + 'static>>;
}
