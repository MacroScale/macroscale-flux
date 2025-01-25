use std::{future::Future, pin::Pin};

#[derive(Clone)]
pub struct TaskMeta {
    pub id: u8,
    pub name: &'static str,
}

pub trait Task: Send {
   fn data(&self) -> &TaskMeta;
   fn execute(self: Box<Self>) -> Pin<Box<dyn Future<Output = ()> + 'static>>;
}
