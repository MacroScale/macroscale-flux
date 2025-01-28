/*
------------------------------------
            Capture Handler 
------------------------------------
Handles the capturing of frames from the game window.
The handler will capture frames at a set frame rate and store them in a buffer.
*/

use std::{collections::VecDeque, error::Error, option, sync::Arc, time::Duration};

use tokio::{sync::Mutex, task::{spawn_local, JoinHandle}, time};

use crate::base::{app_data::AppData, event_loop::{EventDispatcher, EventLoop}, task::{Task, TaskMeta}};

use super::{capture_buffer::CaptureBuffer, task_handler::TaskHandler};

use windows::{
    Win32::Foundation::*,
    Graphics::Capture::*,
};

pub struct CaptureHandler {
    game_hwnd: Arc<Mutex<Option<HWND>>>,
    capture_buffer: Arc<Mutex<Option<CaptureBuffer>>>,
    /// per second
    frame_rate: u64,
    /// in seconds
    capture_length: u64,
}


impl CaptureHandler {
    pub fn new() -> Arc<CaptureHandler> {
        Arc::new(CaptureHandler { 
            game_hwnd: Arc::new(Mutex::new(None)),
            capture_buffer: Arc::new(Mutex::new(None)),
            frame_rate: 60,
            capture_length: 10,
        })
    }

    pub async fn create_buffer(&self, hwnd: Option<HWND>) {
        let buffer = CaptureBuffer::new(hwnd);

        if buffer.is_none() { 
            log::error!("Error creating capture buffer");
            return; 
        }
        
        let mut buffer_ref = self.capture_buffer.lock().await;
        *buffer_ref = buffer;
    }

    pub async fn is_buffer_empty(&self) -> bool {
        let buffer = self.capture_buffer.lock().await;
        buffer.is_some()
    }
   
    pub async fn get_game_hwnd(&self) -> Option<HWND> {
        let game_hwnd = self.game_hwnd.lock().await;
        game_hwnd.clone()
    }

    pub async fn set_game_hwnd(cap_handler: Arc<CaptureHandler>, new_hwnd: Option<HWND>) {
        let mut hwnd_ref = cap_handler.game_hwnd.lock().await;
        *hwnd_ref = new_hwnd;
    }

    pub async fn add_frame(&self, frame: Option<i32>) {
        log::info!("adding frame to buffer");
    }

    async fn pop_frame(&self) {
        log::info!("removing frame from back of buffer");
    }

    async fn clean_buffer(&self){
        log::info!("cleaning buffer");
    }
}

pub async fn start(app_data: Arc<AppData>, task_handler: Arc<TaskHandler>, event_loop: Arc<EventLoop>, dispatcher: EventDispatcher, cap_handler: Arc<CaptureHandler>) {
    log::info!("starting capture handler");
    loop {
        time::sleep(Duration::from_millis(5)).await;

        // set game_hwnd to app_data game_hwnd
        let ad_game_hwnd = app_data.get_game_hwnd().await;
        CaptureHandler::set_game_hwnd(cap_handler.clone(), ad_game_hwnd).await;

        // ensure game_hwnd exists before capturing frames
        if cap_handler.get_game_hwnd().await.is_none(){
            // update with if buffer is empty then continue
            // if buffer is not empty, clean buffer
            if cap_handler.is_buffer_empty().await { continue; } 
            else{ cap_handler.clean_buffer(); }
            continue; 
        }

        // ensure buffer is created before capturing frames
        if cap_handler.is_buffer_empty().await { 
            log::info!("creating buffer for capture");
            let hwnd = cap_handler.get_game_hwnd().await;
            cap_handler.create_buffer(hwnd).await; 
        }

        // start capturing frames
        log::info!("capturing frame");
    }
}
