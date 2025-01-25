pub enum Event {
    HotKeyEvent(HotkeyEventData)
}

pub struct HotkeyEventData{
    /// The id of the hotkey
    pub id: u32,  
    /// virtual key codes
    pub vks: u32,
}
