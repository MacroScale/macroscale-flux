use core::task_handler::TaskHandler;
use std::env;

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

    // init task_handler
    let task_handler = TaskHandler::new();

    // Construct a local task set that can run `!Send` futures.
    let local = task::LocalSet::new();

    // Run the local task set.
    local.run_until(async move {
        task::spawn_local(core::application::start(task_handler)).await.unwrap();
    }).await;

    //client::start().await;
    //client::test_polling();
}
