use std::{future::Future, pin::Pin, ptr, sync::Arc, time::Duration};
use tokio::time;
use windows::{
    Win32::Foundation::*,
};

use crate::{base::{app_data::AppData, event::{ChangeGameWindowData, Event, EventType, HotkeyEventData}, event_loop::{EventDispatcher, EventLoop}, task::{Task, TaskMeta}}, core::task_handler::TaskHandler, utils};

pub struct PollGameWindowTask {
    meta: TaskMeta
}

impl PollGameWindowTask {
    pub fn new() -> Box<PollGameWindowTask> {
        let meta = TaskMeta{
            name: "poll_game_window",
        };
        Box::new(PollGameWindowTask { meta })
    }
}

impl Task for PollGameWindowTask{
    fn data(&self) -> &TaskMeta { &self.meta }
    fn execute(self: Box<Self>, app_data: Arc<AppData>,  _task_handler: Arc<TaskHandler>, _event_loop: Arc<EventLoop>, dispatcher: EventDispatcher) -> Pin<Box<dyn Future<Output = ()> + 'static>> { 
        Box::pin(poll_game_window(app_data.clone(), dispatcher))
    }
}


pub async fn poll_game_window(app_data: Arc<AppData>, dispatcher: EventDispatcher){
    loop {
        time::sleep(Duration::from_millis(100)).await;
        unsafe {
            let fore_hwnd = app_data.get_current_hwnd().await;
            let game_hwnd = app_data.get_game_hwnd().await;

            // check if the game window is still running
            // will not change the game window if the game window is still running
            if game_hwnd.is_some(){
                if utils::is_hwnd_alive(game_hwnd) { continue; }

                let event_data = ChangeGameWindowData{ hwnd: None }; 
                let event = Event(EventType::ChangeGameProcessHWND(event_data));
                dispatcher.dispatch(event).await;
                continue;
            } 


            // change game window if the foreground window is not the game window
            if fore_hwnd == game_hwnd { continue; }
            if utils::is_game(fore_hwnd) {
                log::info!("NEW Game window detected");
                let event_data = ChangeGameWindowData{ hwnd: fore_hwnd }; 
                let event = Event(EventType::ChangeGameProcessHWND(event_data));
                dispatcher.dispatch(event).await;
                continue;
            }
        }
    }
}
