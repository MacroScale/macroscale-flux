
use std::{collections::VecDeque, time::Duration};

use tokio::{sync::mpsc::{self, Receiver, Sender}, task::spawn_local, time};

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
        let res = self.sender.send(event).await;
        log::info!("dispatching event: {:?}", res);
    }
}


pub struct EventLoop {
    sender: Sender<Event>,
    reciever: Receiver<Event>,
    event_queue: VecDeque<Event>,
    dispatcher: EventDispatcher,
}


impl EventLoop{
    pub fn new() -> EventLoop{
        // inbound channel, so threads can send event to the loop
        // this means that the event loop will be the receiver
        let (tx, rx) = mpsc::channel(100);

        EventLoop{
            sender: tx.clone(),
            reciever: rx,
            event_queue: VecDeque::new(),
            dispatcher: EventDispatcher::new(tx.clone()),
        } 
    }

    pub fn dispatcher(&self) -> EventDispatcher {
        self.dispatcher.clone()
    }

    async fn poll_events(mut self){
        log::info!("event loop poll started");
        loop {
            while let Some(event) = self.reciever.recv().await {
                log::info!("event received: {:?}", event);
                self.event_queue.push_back(event);
            }
            time::sleep(Duration::from_millis(50)).await;
        }
    }

    pub async fn start(self) { 
        log::info!("starting event loop");

        let poll_events_handle = spawn_local(self.poll_events());
        tokio::join!(poll_events_handle);

        log::info!("exiting event loop");
    }
}
