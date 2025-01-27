
use std::{future::Future, pin::Pin, sync::Arc, time::Duration};
use tokio::time;

use crate::{base::{app_data::AppData, event::EventType, event_loop::{EventDispatcher, EventLoop}, task::{Task, TaskMeta}}, core::task_handler::TaskHandler, processors::hotkey_processor, tasks::one_shot::{capture_gameplay::CaptureGameplayTask, log_process_windows::LogProcessWindowsTask, quit_application::QuitApplicationTask}};

pub struct PollEventsTask {
    meta: TaskMeta
}

impl PollEventsTask {
    pub fn new() -> Box<PollEventsTask> {
        let meta = TaskMeta{
            name: "poll_events",
        };
        Box::new(PollEventsTask{ meta })
    }
}

impl Task for PollEventsTask{
    fn data(&self) -> &TaskMeta { &self.meta }
    fn execute(self: Box<Self>, app_data: Arc<AppData>,  task_handler: Arc<TaskHandler>, event_loop: Arc<EventLoop>, dispatcher: EventDispatcher) -> Pin<Box<dyn Future<Output = ()> + 'static>> { 
        Box::pin(event_handler(app_data.clone(), task_handler.clone(), event_loop.clone(), dispatcher.clone()))
    }
}


async fn event_handler(_app_data: Arc<AppData>, task_handler: Arc<TaskHandler>, event_loop: Arc<EventLoop>, dispatcher: EventDispatcher){
    loop {
        let event = match event_loop.pop_event().await {
            Some(e) => e,
            None => continue,
        };

        match event.0 {
            EventType::Hotkey(data) => hotkey_processor::handle_hotkey_event(data, dispatcher.clone()).await,
            EventType::Quit => task_handler.add_task(QuitApplicationTask::new()).await, 
            EventType::Capture => task_handler.add_task(CaptureGameplayTask::new()).await, 
            EventType::LogProcessWindows => task_handler.add_task(LogProcessWindowsTask::new()).await, 
            EventType::ChangeForegroundProcessHWND(data) => {},
        }

        time::sleep(Duration::from_millis(50)).await;
    }
}
