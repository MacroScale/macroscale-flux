use crate::base::{event::{Event, EventType, HotkeyEventData}, event_loop::EventDispatcher};


pub async fn process_hotkey_event(data: HotkeyEventData, dispatcher: EventDispatcher) {

    log::info!("handling hotkey event: id={} vks={}", data.id, data.vks);

    match data.id {
        1 => { dispatcher.dispatch(Event(EventType::Quit)).await; }
        2 => { dispatcher.dispatch(Event(EventType::Capture)).await; }
        3 => { dispatcher.dispatch(Event(EventType::LogProcessWindows)).await; }
        _ => { log::error!("no action for hotkey event: id={} vks={}", data.id, data.vks); }
    }
}
