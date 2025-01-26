use crate::base::{event::{ActionEventData, Event}, event_loop::EventDispatcher};

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
        1 => { dispatcher.dispatch(Event::ActionEvent(ActionEventData{ id: 1, name: "quit_action".to_string()})).await; }
        2 => { dispatcher.dispatch(Event::ActionEvent(ActionEventData{ id: 2, name: "capture_action".to_string()})).await; }
        _ => { log::error!("no action for hotkey event: id={} vks={}", data.id, data.vks); }
    }
}
