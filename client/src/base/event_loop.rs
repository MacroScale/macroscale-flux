
use std::collections::VecDeque;

use tokio::sync::mpsc::{self, Sender, Receiver};

use crate::base::event::Event;

pub struct EventLoop {
    sender: Sender<Event>,
    reciever: Receiver<Event>,
    event_queue: VecDeque<Event>
}

impl EventLoop{
    pub fn new() -> EventLoop{
        // inbound channel, so threads can send event to the loop
        // this means that the event loop will be the receiver
        let (tx, rx) = mpsc::channel(100);

        EventLoop{
            sender: tx,
            reciever: rx,
            event_queue: VecDeque::new()
        } 
    }
    pub fn sender(&self) -> Sender<Event> {
        self.sender.clone()
    }

    pub async fn start() { 
    }
}
