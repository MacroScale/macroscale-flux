
use std::{cell::RefCell, future::Future, pin::Pin, ptr, rc::Rc, sync::Arc, time::Duration};
use tokio::{sync::{mpsc::Sender, Mutex}, time};

use crate::base::{event::Event, event_loop::{EventDispatcher, EventLoop}, task::{Task, TaskMeta}};

pub struct EventHandlerTask {
    meta: TaskMeta
}

impl EventHandlerTask {
    pub fn new() -> Box<EventHandlerTask> {
        let meta = TaskMeta{
            id: 0,
            name: "event_handler",
        };
        Box::new(EventHandlerTask{ meta })
    }
}

impl Task for EventHandlerTask{
    fn data(&self) -> &TaskMeta { &self.meta }
    fn execute(self: Box<Self>, event_loop: Arc<EventLoop>, dispatcher: EventDispatcher) -> Pin<Box<dyn Future<Output = ()> + 'static>> { 
        Box::pin(event_processor(event_loop.clone()))
    }
}


async fn event_processor(event_loop: Arc<EventLoop>){
    loop {
        let event = event_loop.pop_event().await;

        if let Some(e) = event {
            log::info!("Processing event: {:?}", e);
        }

        time::sleep(Duration::from_millis(50)).await;
    }
}
