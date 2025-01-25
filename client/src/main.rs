use std::env;

use tokio::task;

//use win_x86_64;

mod client;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // init logging
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    // Construct a local task set that can run `!Send` futures.
    let local = task::LocalSet::new();

    // Run the local task set.
    local.run_until(async move {
        // `spawn_local` ensures that the future is spawned on the local
        // task set.
        task::spawn_local(client::start()).await.unwrap();
    }).await;

    //client::start().await;
    //client::test_polling();
}
