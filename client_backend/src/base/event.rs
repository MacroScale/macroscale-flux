use windows::{
    Win32::Foundation::*,
};

#[derive(Debug)]
pub struct Event(pub EventType);

#[derive(Debug)]
pub enum EventType {
    Hotkey(HotkeyEventData),
    Quit,
    StartCapture,
    StopCapture,
    LogProcessWindows,
    ChangeForegroundProcessHWND(ChangeForegroundWindowData),
    ChangeGameProcessHWND(ChangeGameWindowData),
}

#[derive(Debug)]
pub struct HotkeyEventData{
    /// The id of the hotkey
    pub id: u32,  
    /// virtual key codes
    pub vks: u32,
}

#[derive(Debug)]
pub struct ChangeForegroundWindowData {
    pub hwnd: Option<HWND>,
}

#[derive(Debug)]
pub struct ChangeGameWindowData {
    pub hwnd: Option<HWND>,
}

