use std::env;

//use win_x86_64;

mod client;

#[tokio::main]
async fn main() {
    // init logging
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    client::start().await;
}
