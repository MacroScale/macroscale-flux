use windows::{
    Win32::Foundation::*,
};

#[derive(Debug)]
pub struct Event(pub EventType);

#[derive(Debug)]
pub enum EventType {
    Hotkey(HotkeyEventData),
    Quit,
    Capture,
    LogProcessWindows,
    ChangeForegroundProcessHWND(ActionChangeForegroundProcessHWNDData),
}

#[derive(Debug)]
pub struct HotkeyEventData{
    /// The id of the hotkey
    pub id: u32,  
    /// virtual key codes
    pub vks: u32,
}

#[derive(Debug)]
pub struct ActionChangeForegroundProcessHWNDData{
    /// The id of the action hotkey
    pub id: u32,  
    pub name: String,  
    pub hwnd: Option<HWND>,
}
