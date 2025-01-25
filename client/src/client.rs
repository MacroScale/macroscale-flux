use std::time::Duration;

use tokio::{sync::{mpsc, watch}, time};

pub enum Action {
    Quit,
    CaptureGameplay,
    UploadCapture,
}

pub fn get_action(input: char) -> Option<Action>{
    match input {
        'q' => Some(Action::Quit),
        'w' => Some(Action::CaptureGameplay),
        'e' => Some(Action::UploadCapture),
        _ => None
    }
}

pub async fn capture_action(tx: mpsc::Sender<Action>, shutdown_rx: watch::Receiver<bool>) {
    log::info!("starting capture action service");
    let mut interval = time::interval(Duration::from_millis(50));

    loop {

        interval.tick().await;

        if *shutdown_rx.borrow() {
            log::info!("shutdown signal received: exiting capture action service");
            break;
        }

        // get keyboard input from os interrupt
        let input = 'q';

        if let Some(action) = get_action(input) {
            if let Err(err) = tx.send(action).await {
                log::error!("Failed to send action: {}", err);
            }
        }
    }
}

pub async fn execute_action(mut rx: mpsc::Receiver<Action>, shutdown_tx: watch::Sender<bool>){
    log::info!("starting exectute action service");
    while let Some(action) = rx.recv().await {
        match action {
            Action::Quit => { let _ = shutdown_tx.send(true); },
            Action::CaptureGameplay => log::info!("Capturing gameplay..."),
            Action::UploadCapture => log::info!("Uploading gameplay..."),
        };

        if *shutdown_tx.borrow() {
            log::info!("shutdown signal received: exiting execute action service");
            break;
        }
    }
}

pub async fn start(){
    log::info!("Starting Macroscale Game Capture");

    let (tx, rx) = mpsc::channel(100);
    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    let capture_action_handle = tokio::spawn(capture_action(tx, shutdown_rx.clone()));
    let execute_action_handle = tokio::spawn(execute_action(rx, shutdown_tx));

    // Wait for all tasks to terminate after shutdown is triggered
    let _ = tokio::join!(capture_action_handle, execute_action_handle);

    log::info!("Exiting Macroscale Game Capture");
}
