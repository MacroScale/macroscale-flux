
use std::{collections::VecDeque, sync::Arc, time::Duration};

use tokio::{sync::{mpsc::{self, Receiver, Sender}, Mutex}, task::spawn_local, time};

use crate::base::event::Event;

#[derive(Clone)]
pub struct EventDispatcher {
    sender: Sender<Event>,
}

impl EventDispatcher {
    pub fn new(sender: Sender<Event>) -> EventDispatcher {
        EventDispatcher { sender }
    }

    pub async fn dispatch(&self, event: Event) {
        log::info!("dispatching event: {:?}", event);
        let _ = self.sender.send(event).await;
    }
}

pub struct EventLoop {
    reciever: Arc<Mutex<Receiver<Event>>>,
    event_queue: Arc<Mutex<VecDeque<Event>>>,
}

impl EventLoop{
    pub fn new() -> (Arc<EventLoop>, EventDispatcher) {
        // dispatch channel, so threads can send event to the loop
        let (tx, rx) = mpsc::channel(100);

        let event_loop = Arc::new(
            EventLoop {
                reciever: Arc::new(Mutex::new(rx)),
                event_queue: Arc::new(Mutex::new(VecDeque::new())),
        });

        (
            event_loop,
            EventDispatcher::new(tx)
        )
    }

    // functions for self 
    pub async fn push_event(&self, event: Event) {
        self.event_queue.lock().await.push_back(event);
    }

    pub async fn pop_event(&self) -> Option<Event> {
        self.event_queue.lock().await.pop_front()
    }

    // functions for shared ref
    async fn poll_inbound_events(event_loop: Arc<EventLoop>){
        loop {
            let ev_rec_ref = event_loop.reciever.clone();
            let mut reciever = ev_rec_ref.lock().await;

            while let Some(event) = reciever.recv().await {
                event_loop.push_event(event).await;
            }

            drop(reciever);
            time::sleep(Duration::from_millis(50)).await;
        }
    }

    pub async fn start(event_loop: Arc<EventLoop>) { 
        log::info!("starting event loop");

        let poll_events_handle = spawn_local(Self::poll_inbound_events(event_loop.clone()));
        let _ = tokio::join!(poll_events_handle);

        log::info!("exiting event loop");
    }
}


