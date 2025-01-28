use std::{future::Future, pin::Pin, sync::Arc};

use crate::{base::{app_data::AppData, event_loop::{EventDispatcher, EventLoop}, task::{Task, TaskMeta}}, core::task_handler::TaskHandler, tasks::one_shot::quit_application::QuitApplicationTask};

use windows::{
    Win32::Foundation::*,
    Graphics::Capture::*,
};

pub struct CheckGCSTask {
    meta: TaskMeta
}

impl CheckGCSTask {
    pub fn new() -> Box<CheckGCSTask> {
        let meta = TaskMeta {
            name: "check_graphics_capture_session",
        };
        Box::new(CheckGCSTask{ meta })
    }
}

impl Task for CheckGCSTask {
    fn data(&self) -> &TaskMeta { &self.meta }
    fn execute(self: Box<Self>, app_data: Arc<AppData>, task_handler: Arc<TaskHandler>, _event_loop: Arc<EventLoop>, _dispatcher: EventDispatcher) -> Pin<Box<dyn Future<Output = ()> + 'static>> { 
        Box::pin(check_graphics_capture_session(task_handler))
    }
}

async fn check_graphics_capture_session(th: Arc<TaskHandler>) {
    unsafe {
        let is_supported_res = GraphicsCaptureSession::IsSupported();
        match is_supported_res {
            Ok(is_supported) => { log::info!("success: GraphicsCaptureSession is supported") },
            Err(e) => {
                log::info!("GraphicsCaptureSession is not supported: {:?}", e);
                th.add_task(QuitApplicationTask::new()).await; 
            },
        }
    }
}
