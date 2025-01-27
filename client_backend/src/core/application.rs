use std::{slice::Windows, sync::Arc};

use tokio::task::spawn_local;

use crate::{base::{app_data::AppData, event_loop::{EventDispatcher, EventLoop}}, core::task_handler, tasks::{one_shot::register_hotkeys::RegisterHotkeysTask, pollers::{poll_events::PollEventsTask, poll_foreground_window::PollForegroundWindowTask, poll_hotkey::PollHotkeysTask}}};

use super::task_handler::TaskHandler;

pub async fn start(app_data: Arc<AppData>, th: Arc<TaskHandler>, event_loop: Arc<EventLoop>, dispatcher: EventDispatcher) { 

    // start event_loop
    let event_loop_handle = spawn_local(EventLoop::start(event_loop.clone()));

    // initialise core tasks
    // oneshot
    let register_hotkeys_task = RegisterHotkeysTask::new();

    // pollers 
    let poll_events_task = PollEventsTask::new();
    let poll_hotkeys_task = PollHotkeysTask::new();
    let poll_foreground_window_task = PollForegroundWindowTask::new();

    // add tasks to task_handler
    th.add_task(register_hotkeys_task).await;
    th.add_task(poll_events_task).await;
    th.add_task(poll_hotkeys_task).await;
    th.add_task(poll_foreground_window_task).await;


    // start task_handler
    let task_handler_handle = spawn_local(
        task_handler::start(app_data.clone(), th.clone(), event_loop.clone(), dispatcher)
    );

    let _ = tokio::join!(event_loop_handle, task_handler_handle);

    log::info!("exiting application");
}

