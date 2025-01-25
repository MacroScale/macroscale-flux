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

pub enum Action {
    Quit,
    CaptureGameplay,
    UploadCapture,
}

/// register_hotkeys
/// will be running headless, meaning no window handle supplied
/// This also means that the only way to recieve input is to poll
/// the message queue and check if a hotkey was pressed
pub fn register_hotkeys() -> Result<()> {
    log::info!("Registering hotkeys");
    unsafe {
        let quit_hotkey = RegisterHotKey(
            Some(HWND(ptr::null_mut())),
            1,
            MOD_ALT,
            'Q' as u32,
        );
        log::info!("Registered Quit Hotkey: ALT + SHIFT + Q")
    };
    Ok(())
}

pub async fn poll_hotkeys(shutdown_rx: watch::Receiver<bool>){
    log::info!("starting polling hotkeys service");
    let mut interval = time::interval(Duration::from_millis(50));

    loop {
        unsafe {
            let mut msg = MSG {
                hwnd: HWND(ptr::null_mut()),
                message: 0,
                wParam: WPARAM(0),
                lParam: LPARAM(0),
                time: 0,
                pt: std::mem::zeroed(),
            };

            let peek_value = PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool();
            if  peek_value {
                if msg.message == WM_HOTKEY {
                    log::info!("Win + s pressed!");
                }
                log::info!("Peek value {} Message Value: {}", peek_value, msg.message);
            }
        }
        if *shutdown_rx.borrow() {
            log::info!("Shutdown signal received: exiting hotkeys polling service");
            break;
        }
        interval.tick().await;
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
    }
}

pub async fn execute_action(mut rx: mpsc::Receiver<Action>, shutdown_tx: watch::Sender<bool>){
    log::info!("starting execute action service");

    loop {
        let action_res = rx.recv().await;

        let action = match action_res{
            Some(a) => a,
            None => {
                log::error!("channel closed: exiting execute action service");
                return;
            } 
        };

        match action {
            Action::Quit => { let _ = shutdown_tx.send(true); },
            Action::CaptureGameplay => log::info!("Capturing gameplay..."),
            Action::UploadCapture => log::info!("Uploading gameplay..."),
        };

        if *shutdown_tx.borrow() {
            log::info!("shutdown signal received: exiting execute action service");
            return;
        }
    }
}

pub async fn start(){
    log::info!("Starting Macroscale Game Capture");
    let _ = register_hotkeys();

    let (tx, rx) = mpsc::channel(100);
    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    let poll_hotkeys_handle = spawn_local(poll_hotkeys(shutdown_rx.clone()));
    let capture_action_handle = spawn_local(capture_action(tx, shutdown_rx.clone()));
    let execute_action_handle = spawn_local(execute_action(rx, shutdown_tx));

    // Wait for all tasks to terminate after shutdown is triggered
    let _ = tokio::join!(
        poll_hotkeys_handle,
        capture_action_handle,
        execute_action_handle
    );

    log::info!("Exiting Macroscale Game Capture");
}
