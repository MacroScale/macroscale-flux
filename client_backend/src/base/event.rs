use windows::{
    Win32::Foundation::*,
};

pub enum Event {
    HotKeyEvent(EventData),
    ActionEvent(EventData),
}

pub enum EventData {
    HotKey(HotkeyEventData),
    ActionHotkey(ActionHotkeyEventData),
    ActionChangeForegroundProcessHWND(ActionChangeForegroundProcessHWNDData),
}

pub struct HotkeyEventData{
    /// The id of the hotkey
    pub id: u32,  
    /// virtual key codes
    pub vks: u32,
}

pub struct ActionHotkeyEventData{
    /// The id of the action hotkey
    pub id: u32,  
    pub name: String,  
}

pub struct ActionChangeForegroundProcessHWNDData{
    /// The id of the action hotkey
    pub id: u32,  
    pub name: String,  
    pub hwnd: Option<HWND>,
}
