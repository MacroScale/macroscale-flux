use std::{future::Future, pin::Pin, sync::Arc};

use crate::{base::{app_data::AppData, event_loop::{EventDispatcher, EventLoop}, task::{Task, TaskMeta}}, core::task_handler::TaskHandler, tasks::one_shot::quit_application::QuitApplicationTask};

use windows::{
    Win32::Foundation::*,
    Win32::System::SystemInformation::GetVersion,
    Graphics::Capture::*,
};

pub struct CheckWinVerTask {
    meta: TaskMeta
}

impl CheckWinVerTask {
    pub fn new() -> Box<CheckWinVerTask> {
        let meta = TaskMeta {
            name: "check_win_version",
        };
        Box::new(CheckWinVerTask{ meta })
    }
}

impl Task for CheckWinVerTask {
    fn data(&self) -> &TaskMeta { &self.meta }
    fn execute(self: Box<Self>, app_data: Arc<AppData>, task_handler: Arc<TaskHandler>, _event_loop: Arc<EventLoop>, _dispatcher: EventDispatcher) -> Pin<Box<dyn Future<Output = ()> + 'static>> { 
        Box::pin(check_win_ver(task_handler))
    }
}

async fn check_win_ver(th: Arc<TaskHandler>) {
    unsafe {
        let win_ver = GetVersion();

        let major_version = (win_ver & 0xFF) as u8;
        let minor_version = ((win_ver >> 8) & 0xFF) as u8;
        let build_number = if win_ver & (1 << 31) == 0 {
            (win_ver >> 16) & 0xFFFF
        } else {
            0 // Older versions may not include a build number
        };

        log::info!(
            "Windows version: {}.{}.{}",
            major_version,
            minor_version,
            build_number
        );

        /*
        match is_supported_res {
            Ok(is_supported) => { log::info!("success: GraphicsCaptureSession is supported") },
            Err(e) => {
                log::info!("GraphicsCaptureSession is not supported: {:?}", e);
                th.add_task(QuitApplicationTask::new()).await; 
            },
        }
        */
    }
}
