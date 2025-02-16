use std::{future::Future, pin::Pin, sync::Arc};

use crate::{base::{app_data::AppData, event::{Event, EventType}, event_loop::{EventDispatcher, EventLoop}, task::{Task, TaskMeta}}, core::task_handler::TaskHandler};

pub struct StopCaptureTask {
    meta: TaskMeta
}

impl StopCaptureTask {
    pub fn new() -> Box<StopCaptureTask> {
        let meta = TaskMeta {
            name: "stop_capture",
        };
        Box::new(StopCaptureTask{ meta })
    }
}

impl Task for StopCaptureTask {
    fn data(&self) -> &TaskMeta { &self.meta }
    fn execute(self: Box<Self>, app_data: Arc<AppData>, _task_handler: Arc<TaskHandler>, _event_loop: Arc<EventLoop>, dispatcher: EventDispatcher) -> Pin<Box<dyn Future<Output = ()> + 'static>> { 
        Box::pin(stop_capture(app_data, dispatcher))
    }
}

async fn stop_capture(app_data: Arc<AppData>, dispatcher: EventDispatcher) {

    if let Some(session) = &(*AppData::get_capture_session(app_data.clone()).await.lock().await) {
        session.Close();
        let save_event = Event(EventType::SaveSessionToVideo);
        dispatcher.dispatch(save_event);
        return;
    }

    log::error!("stop_capture failed: no capture session started");
}
