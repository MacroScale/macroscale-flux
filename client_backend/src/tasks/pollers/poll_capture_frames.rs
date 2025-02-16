use std::{future::Future, pin::Pin, sync::Arc, time::Duration};
use tokio::time;

use crate::{base::{app_data::AppData, event::EventType, event_loop::{EventDispatcher, EventLoop}, task::{Task, TaskMeta}}, core::task_handler::TaskHandler, processors::hotkey_processor, tasks::one_shot::{log_process_windows::LogProcessWindowsTask, quit_application::QuitApplicationTask, start_capture::StartCaptureTask, stop_capture::StopCaptureTask}};

pub struct PollCaptureFramesTask {
    meta: TaskMeta
}

impl PollCaptureFramesTask {
    pub fn new() -> Box<PollCaptureFramesTask> {
        let meta = TaskMeta{
            name: "poll_events",
        };
        Box::new(PollCaptureFramesTask{ meta })
    }
}

impl Task for PollCaptureFramesTask{
    fn data(&self) -> &TaskMeta { &self.meta }
    fn execute(self: Box<Self>, app_data: Arc<AppData>,  task_handler: Arc<TaskHandler>, event_loop: Arc<EventLoop>, dispatcher: EventDispatcher) -> Pin<Box<dyn Future<Output = ()> + 'static>> { 
        Box::pin(poll_caputure_frames(app_data.clone()))
    }
}


// events MUST be processed synchronously, unlike async tasks
async fn poll_caputure_frames(app_data: Arc<AppData>){
    loop {

        time::sleep(Duration::from_millis(50)).await;
    }
}
