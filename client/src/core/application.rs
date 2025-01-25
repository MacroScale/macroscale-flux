use tokio::task::spawn_local;

use crate::tasks::{poll_hotkeys::PollHotkeysTask, register_hotkeys::RegisterHotkeysTask};

use super::task_handler::TaskHandler;


pub async fn start(mut task_handler: TaskHandler) { 
    log::info!("Starting Macroscale Game Capture");

    // initialise all tasks
    let register_hotkeys_task = RegisterHotkeysTask::new();
    let poll_hotkeys_task = PollHotkeysTask::new();

    // add tasks to task_handler
    task_handler.add_task(register_hotkeys_task);
    task_handler.add_task(poll_hotkeys_task);

    // start task_handler
    let task_handler_handle = spawn_local(task_handler.start());

    tokio::join!(task_handler_handle);

    log::info!("Exiting Macroscale Game Capture");
}

