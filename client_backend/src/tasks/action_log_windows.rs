use std::{future::Future, pin::Pin, sync::Arc};

use crate::{base::{app_data::AppData, event_loop::{EventDispatcher, EventLoop}, task::{Task, TaskMeta}}, core::task_handler::TaskHandler, utils::hwnd_to_string};

use windows::{
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
};

pub struct ActionLogWindowsTask {
    meta: TaskMeta
}

impl ActionLogWindowsTask {
    pub fn new() -> Box<ActionLogWindowsTask> {
        let meta = TaskMeta {
            name: "action_log_windows"
        };
        Box::new(ActionLogWindowsTask{ meta })
    }
}

impl Task for ActionLogWindowsTask {
    fn data(&self) -> &TaskMeta { &self.meta }
    fn execute(self: Box<Self>, app_data: Arc<AppData>, _task_handler: Arc<TaskHandler>, _event_loop: Arc<EventLoop>, _dispatcher: EventDispatcher) -> Pin<Box<dyn Future<Output = ()> + 'static>> { 
        Box::pin(action_log_windows())
    }
}


async fn action_log_windows() {

    let mut hwnd_list: Vec<HWND> = Vec::new();
    let mut win_titles: Vec<String> = Vec::new();

    unsafe extern "system" fn callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
        // Cast LPARAM back to a mutable reference to Vec<HWND>
        let win_list = &mut *(lparam.0 as *mut Vec<HWND>);
        win_list.push(hwnd);
        BOOL::from(true)
    }

    unsafe {
        // Pass a mutable reference to `hwnd_list` through LPARAM
        EnumWindows(Some(callback), LPARAM(&mut hwnd_list as *mut _ as isize))
            .expect("Failed to get windows");

        // Process handles to win titles 
        for handle in hwnd_list.iter() {
            let mut title = hwnd_to_string(*handle);
            if let Some(t) = title { win_titles.push(t); }
        }
    }
    log::info!("windows titles: {:?}", win_titles);
}
