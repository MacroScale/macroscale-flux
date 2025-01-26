use std::{slice::Windows, sync::Arc};

use tokio::task::spawn_local;

use crate::{base::{app_data::AppData, event_loop::{EventDispatcher, EventLoop}}, core::task_handler, tasks::{event_handler::EventHandlerTask, poll_foreground_window::{self, PollForegroundWindowTask}, poll_hotkeys::PollHotkeysTask, register_hotkeys::RegisterHotkeysTask, window_capturer::WindowCapturerTask}};

use super::task_handler::TaskHandler;

pub async fn start(app_data: Arc<AppData>, th: Arc<TaskHandler>, event_loop: Arc<EventLoop>, dispatcher: EventDispatcher) { 

    // initialise core tasks
    let register_hotkeys_task = RegisterHotkeysTask::new();
    let poll_hotkeys_task = PollHotkeysTask::new();
    let event_handler_task = EventHandlerTask::new();
    let window_capture_task = WindowCapturerTask::new();
    let poll_foreground_window_task = PollForegroundWindowTask::new();

    // add tasks to task_handler
    th.add_task(register_hotkeys_task).await;
    th.add_task(poll_hotkeys_task).await;
    th.add_task(event_handler_task).await;
    th.add_task(window_capture_task).await;
    th.add_task(poll_foreground_window_task).await;

    // start event_loop
    let event_loop_handle = spawn_local(EventLoop::start(event_loop.clone()));

    // start task_handler
    let task_handler_handle = spawn_local(
        task_handler::start(app_data.clone(), th.clone(), event_loop.clone(), dispatcher)
    );

    let _ = tokio::join!(event_loop_handle, task_handler_handle);

    log::info!("exiting application");
}

