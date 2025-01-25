use std::{ptr, time::Duration};

use tokio::sync::{mpsc, watch};
use tokio::task::spawn_local;
use tokio::time;

use windows::{
    core::Result,
    Win32::Foundation::*,
    Win32::UI::Input::KeyboardAndMouse::*,
    Win32::UI::WindowsAndMessaging::*,
};

/*
pub enum Action {
    Quit,
    CaptureGameplay,
    UploadCapture,
}
*/

pub async fn start(){
    log::info!("Starting Macroscale Game Capture");
    let _ = register_hotkeys();

    let (tx, rx) = mpsc::channel(100);
    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    let poll_hotkeys_handle = spawn_local(poll_hotkeys(shutdown_rx.clone()));
    let capture_action_handle = spawn_local(capture_action(tx, shutdown_rx.clone()));
    let execute_action_handle = spawn_local(execute_action(rx, shutdown_tx));

    // Wait for all tasks to terminate after shutdown is triggered
    let test = tokio::join!(
        poll_hotkeys_handle,
        capture_action_handle,
        execute_action_handle
    );

    log::info!("Exiting Macroscale Game Capture");
}
