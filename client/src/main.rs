use core::task_handler::TaskHandler;
use std::env;

use base::event_loop::EventLoop;
use tokio::task;

mod base;
mod core;
mod tasks;
mod processors;
mod api;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // init logging
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    // init event loop
    let (event_loop, event_dispatcher) = EventLoop::new();

    // init task_handler
    let task_handler = TaskHandler::new();

    // construct a local task set that can run `!Send` futures.
    // used for single-threaded runtime (current_thread)
    let local = task::LocalSet::new();

    local.run_until(async move {
        //let event_loop_task = task::spawn_local(EventLoop::start(event_loop.clone()));
        let application_handle = task::spawn_local(
            core::application::start(
                task_handler.clone(),
                event_loop.clone(),
                event_dispatcher.clone()
            )
        );

        let _ = tokio::join!(application_handle);
    }).await;
}
