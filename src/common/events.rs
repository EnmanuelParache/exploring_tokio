use std::sync::Arc;
use tokio::sync::broadcast::{Receiver, Sender};

#[derive(Debug, Clone)]
pub enum Events {
    Start,
    TaskAFinished,
    TaskBFinished,
    TaskCFinished,
    Complete,
}

#[derive(Debug)]
pub struct EventChannel {
    pub tx: Arc<Sender<Events>>,
    pub rx: Receiver<Events>,
}

impl EventChannel {
    pub fn new(txr: Arc<Sender<Events>>) -> EventChannel {
        EventChannel {
            tx: txr.clone(),
            rx: txr.clone().subscribe(),
        }
    }
}
