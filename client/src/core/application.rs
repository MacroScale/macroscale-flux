use std::{cell::RefCell, rc::Rc, sync::Arc};

use tokio::{sync::Mutex, task::spawn_local};

use crate::{base::event_loop::{EventDispatcher, EventLoop}, tasks::{event_handler::EventHandlerTask, poll_hotkeys::PollHotkeysTask, register_hotkeys::RegisterHotkeysTask}};

use super::task_handler::TaskHandler;

pub async fn start(mut task_handler: TaskHandler, event_loop: Arc<EventLoop>, dispatcher: EventDispatcher) { 
    // initialise all tasks
    let register_hotkeys_task = RegisterHotkeysTask::new();
    let poll_hotkeys_task = PollHotkeysTask::new();
    let event_handler_task = EventHandlerTask::new();

    // add tasks to task_handler
    task_handler.add_task(register_hotkeys_task);
    task_handler.add_task(poll_hotkeys_task);
    task_handler.add_task(event_handler_task);

    // start task_handler
    let task_handler_handle = spawn_local(task_handler.start(event_loop.clone(), dispatcher));

    tokio::join!(task_handler_handle);

    log::info!("exiting application");
}

