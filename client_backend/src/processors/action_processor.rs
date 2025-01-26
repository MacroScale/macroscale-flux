use std::sync::Arc;

use crate::{base::event::Event, core::task_handler::TaskHandler, tasks::{action_capture::ActionCaptureTask, action_log_windows::ActionLogWindowsTask, action_quit::ActionQuitTask}};


pub async fn handle_action_event(event: Event, task_handler: Arc<TaskHandler>) {

    let data = match event {
        Event::ActionEvent(e) => { e } 
        _ => { 
            log::error!("event not handled");
            return
        }
    };

    log::info!("handling action event: {}", data.name);

    // match key to action and dispatch action event to the event loop 

    match data.name.as_str() {
        "action_quit" => {
            let quit_task = ActionQuitTask::new();
            task_handler.add_task(quit_task).await; 
        }
        "action_log_windows" => {
            let print_windows_task = ActionLogWindowsTask::new();
            task_handler.add_task(print_windows_task).await; 
        }
        "action_capture" => {
            let capture_task = ActionCaptureTask::new();
            task_handler.add_task(capture_task).await; 
        }
        _ => { log::error!("no function for action: id={} name={} ", data.id, data.name); }
    };
}
