use std::{future::Future, pin::Pin, ptr, sync::Arc, time::Duration};
use tokio::time;
use windows::{
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
};

use crate::{base::{app_data::AppData, event_loop::{EventDispatcher, EventLoop}, task::{Task, TaskMeta}}, core::task_handler::TaskHandler, utils};

pub struct PollForegroundWindowTask {
    meta: TaskMeta
}

impl PollForegroundWindowTask {
    pub fn new() -> Box<PollForegroundWindowTask> {
        let meta = TaskMeta{
            name: "poll_foreground_window",
        };
        Box::new(PollForegroundWindowTask { meta })
    }
}

impl Task for PollForegroundWindowTask{
    fn data(&self) -> &TaskMeta { &self.meta }
    fn execute(self: Box<Self>, app_data: Arc<AppData>,  _task_handler: Arc<TaskHandler>, _event_loop: Arc<EventLoop>, dispatcher: EventDispatcher) -> Pin<Box<dyn Future<Output = ()> + 'static>> { 
        Box::pin(poll_foreground(app_data.clone(), dispatcher))
    }
}


pub async fn poll_foreground(app_data: Arc<AppData>, dispatcher: EventDispatcher){
    loop {
        unsafe {
            let previous_hwnd_op = app_data.get_current_hwnd().await;
            let foreground_window_op = utils::get_foreground_window_hwnd();

            if previous_hwnd_op.is_none() && foreground_window_op.is_some() {

                AppData::set_current_hwnd(
                    app_data.clone(),
                    Some(foreground_window_op.unwrap())
                ).await;
                    let fp = utils::get_file_path_hwnd(foreground_window_op.unwrap());
            }
            else if previous_hwnd_op.is_some() && foreground_window_op.is_none() {
                AppData::set_current_hwnd(
                    app_data.clone(),
                    None
                ).await;
            }
            else if previous_hwnd_op.is_some() && foreground_window_op.is_some() {

                let previous_hwnd = previous_hwnd_op.unwrap();
                let foreground_window = foreground_window_op.unwrap();

                if previous_hwnd != foreground_window {
                    AppData::set_current_hwnd(
                        app_data.clone(),
                        Some(foreground_window)
                    ).await;
                }
            }

        }
        time::sleep(Duration::from_millis(50)).await;
    }
}
