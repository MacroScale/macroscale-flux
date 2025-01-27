use crate::base::{event::{ActionHotkeyEventData, Event}, event_loop::EventDispatcher};

pub async fn handle_hotkey_event(event: Event, dispatcher: EventDispatcher) {

    let data = match event {
        Event::HotKeyEvent(e) => { e } 
        _ => { 
            log::error!("event not handled");
            return
        }
    };

    log::info!("handling hotkey event: id={} vks={}", data.id, data.vks);

    // match key to action and dispatch action event to the event loop 

    match data.id {
        1 => { dispatcher.dispatch(Event::ActionEvent(ActionHotkeyEventData{ id: 1, name: "action_quit".to_string()})).await; }
        2 => { dispatcher.dispatch(Event::ActionEvent(ActionHotkeyEventData{ id: 2, name: "action_capture".to_string()})).await; }
        3 => { dispatcher.dispatch(Event::ActionEvent(ActionHotkeyEventData{ id: 3, name: "action_log_windows".to_string()})).await; }
        _ => { log::error!("no action for hotkey event: id={} vks={}", data.id, data.vks); }
    }
}
