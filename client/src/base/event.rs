pub enum Event {
    HotKeyEvent(HotkeyEventData),
    ActionEvent(ActionEventData),
}

pub struct HotkeyEventData{
    /// The id of the hotkey
    pub id: u32,  
    /// virtual key codes
    pub vks: u32,
}

pub struct ActionEventData{
    /// The id of the action hotkey
    pub id: u32,  
    pub name: String,  
}
