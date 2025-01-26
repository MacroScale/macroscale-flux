use std::ptr;

use windows::{
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
};

pub fn get_foreground_window_hwnd() -> Option<HWND> {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd == HWND(ptr::null_mut()) { return None; }
        return Some(hwnd);
    }
}

pub fn hwnd_to_string(hwnd: HWND) -> Option<String> {
    unsafe {
        let mut title: [u8; 64] = [0; 64];
        let complete = GetWindowTextA(hwnd, &mut title);
        let title = title.iter().position(|&x| x == 0)
            .map_or(&title[..], |i| &title[..i]);
    
        let title_str = String::from_utf8_lossy(title).to_string();
        if title_str.is_empty() { return None; }

        return Some(title_str);
    }
}
