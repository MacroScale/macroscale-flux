
use std::{future::Future, pin::Pin, sync::Arc, time::Duration};
use tokio::time;

use crate::{base::{app_data::AppData, event::Event, event_loop::{EventDispatcher, EventLoop}, task::{Task, TaskMeta}}, core::task_handler::TaskHandler, processors::{action_processor, hotkey_processor}};

pub struct EventHandlerTask {
    meta: TaskMeta
}

impl EventHandlerTask {
    pub fn new() -> Box<EventHandlerTask> {
        let meta = TaskMeta{
            name: "event_handler",
        };
        Box::new(EventHandlerTask{ meta })
    }
}

impl Task for EventHandlerTask{
    fn data(&self) -> &TaskMeta { &self.meta }
    fn execute(self: Box<Self>, app_data: Arc<AppData>,  task_handler: Arc<TaskHandler>, event_loop: Arc<EventLoop>, dispatcher: EventDispatcher) -> Pin<Box<dyn Future<Output = ()> + 'static>> { 
        Box::pin(event_handler(task_handler.clone(), event_loop.clone(), dispatcher.clone()))
    }
}


async fn event_handler(task_handler: Arc<TaskHandler>, event_loop: Arc<EventLoop>, dispatcher: EventDispatcher){
    loop {
        let event = event_loop.pop_event().await;

        if let Some(e) = event {
            match e {
                Event::HotKeyEvent(_) => hotkey_processor::handle_hotkey_event(e, dispatcher.clone()).await,
                Event::ActionEvent(_) => action_processor::handle_action_event(e, task_handler.clone()).await,
            };

        }

        time::sleep(Duration::from_millis(50)).await;
    }
}
