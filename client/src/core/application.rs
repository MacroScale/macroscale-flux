use std::sync::Arc;

use tokio::task::spawn_local;

use crate::{base::event_loop::{EventDispatcher, EventLoop}, core::task_handler, tasks::{event_handler::EventHandlerTask, poll_hotkeys::PollHotkeysTask, register_hotkeys::RegisterHotkeysTask}};

use super::task_handler::TaskHandler;

pub async fn start(th: Arc<TaskHandler>, event_loop: Arc<EventLoop>, dispatcher: EventDispatcher) { 

    // initialise core tasks
    let register_hotkeys_task = RegisterHotkeysTask::new();
    let poll_hotkeys_task = PollHotkeysTask::new();
    let event_handler_task = EventHandlerTask::new();

    // add tasks to task_handler
    th.add_task(register_hotkeys_task).await;
    th.add_task(poll_hotkeys_task).await;
    th.add_task(event_handler_task).await;

    // start event_loop
    let event_loop_handle = spawn_local(EventLoop::start(event_loop.clone()));

    // start task_handler
    let task_handler_handle = spawn_local(
        task_handler::start(th.clone(), event_loop.clone(), dispatcher)
    );

    let _ = tokio::join!(event_loop_handle, task_handler_handle);

    log::info!("exiting application");
}

