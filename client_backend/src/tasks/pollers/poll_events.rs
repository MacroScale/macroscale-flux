
use std::{future::Future, pin::Pin, sync::Arc, time::Duration};
use tokio::time;

use crate::{base::{app_data::AppData, event::EventType, event_loop::{EventDispatcher, EventLoop}, task::{Task, TaskMeta}}, core::task_handler::TaskHandler, processors::hotkey_processor, tasks::one_shot::{log_process_windows::LogProcessWindowsTask, quit_application::QuitApplicationTask, start_capture::StartCaptureTask, stop_capture::StopCaptureTask}};

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


// events MUST be processed synchronously, unlike async tasks
async fn event_handler(app_data: Arc<AppData>, task_handler: Arc<TaskHandler>, event_loop: Arc<EventLoop>, dispatcher: EventDispatcher){
    loop {
        time::sleep(Duration::from_millis(50)).await;

        let event = match event_loop.pop_event().await {
            Some(e) => e,
            None => { continue; },
        };

        match event.0 {
            EventType::Hotkey(data) => hotkey_processor::process_hotkey_event(data, dispatcher.clone()).await,
            EventType::Quit => task_handler.add_task(QuitApplicationTask::new()).await, 
            EventType::StartCapture => task_handler.add_task(StartCaptureTask::new()).await, 
            EventType::StopCapture => task_handler.add_task(StopCaptureTask::new()).await, 
            EventType::SaveSessionToVideo => task_handler.add_task(StopCaptureTask::new()).await, 
            EventType::LogProcessWindows => task_handler.add_task(LogProcessWindowsTask::new()).await, 

            // could not implement a one-shot task for these events, hwnd cannot be sent across threads
            // TODO: i probably can do this, i will look into this later
            EventType::ChangeForegroundProcessHWND(data) => AppData::set_current_hwnd(app_data.clone(), data.hwnd).await,
            EventType::ChangeGameProcessHWND(data) => AppData::set_game_hwnd(app_data.clone(), data.hwnd).await,
        }
    }
}
