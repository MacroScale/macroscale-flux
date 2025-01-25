use core::task_handler::TaskHandler;
use std::env;

use base::event_loop::EventLoop;
use tokio::task;

//use win_x86_64;

mod base;
mod core;
mod tasks;
mod api;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // init logging
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    // init event loop
    let event_loop = EventLoop::new();

    // init task_handler
    let task_handler = TaskHandler::new(event_loop.sender());

    // Construct a local task set that can run `!Send` futures.
    let local = task::LocalSet::new();

    // Run the local task set.
    local.run_until(async move {
        task::spawn_local(core::application::start(task_handler)).await.unwrap();
        task::spawn_local(event_loop.start()).await.unwrap();
    }).await;

    //client::start().await;
    //client::test_polling();
}
