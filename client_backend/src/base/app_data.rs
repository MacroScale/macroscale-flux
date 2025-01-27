use std::sync::Arc;

use tokio::sync::Mutex;

use windows::{
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
};

pub struct AppDataSettings{
    pub hotkey_quit: u32,
    pub hotkey_capture: u32,
    pub hotkey_log_windows: u32,
}

impl Default for AppDataSettings{
    fn default() -> Self {
        AppDataSettings {
            hotkey_quit: 0,
            hotkey_capture: 0,
            hotkey_log_windows: 0,
        }
    }
}

pub struct AppData {
    current_window_hwnd: Mutex<Option<HWND>>,
    current_game_hwnd: Mutex<Option<HWND>>,
    settings: Mutex<AppDataSettings>,
}

impl AppData {
    pub fn new() -> Arc<AppData> {
        Arc::new(AppData {
            current_window_hwnd: Mutex::new(None),
            current_game_hwnd: Mutex::new(None),
            settings: Mutex::new(AppDataSettings::default()),
        })
    }

    pub async fn get_current_hwnd(&self) -> Option<HWND> {
        let current_window_hwnd = self.current_window_hwnd.lock().await;
        current_window_hwnd.clone()
    }

    pub async fn set_current_hwnd(app_data: Arc<AppData>, hwnd: Option<HWND>) {
        let mut data_ref = app_data.current_window_hwnd.lock().await;
        *data_ref = hwnd;
    }

    pub async fn set_game_hwnd(app_data: Arc<AppData>, hwnd: Option<HWND>) {
        let mut data_ref = app_data.current_game_hwnd.lock().await;
        *data_ref = hwnd;
    }
}
