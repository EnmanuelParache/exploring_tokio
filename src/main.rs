use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;

mod common {
    pub mod events;
}
use common::events::Events;
use common::events::{EventChannel, Events::*};

mod modules {
    pub mod module_a;
    pub mod module_b;
    pub mod module_c;
}
use modules::{module_a::init_task_a, module_b::init_task_b, module_c::init_task_c};

extern crate syslog;
#[macro_use]
extern crate log;

use log::LevelFilter;
use syslog::{BasicLogger, Facility, Formatter3164};

#[tokio::main]
async fn main() {
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "exploring_tokio".into(),
        pid: 0,
    };

    let logger = syslog::unix(formatter).expect("could not connect to syslog");
    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
        .map(|()| log::set_max_level(LevelFilter::Info))
        .expect("could not set boxed logger");

    let (tx, mut rx) = broadcast::channel(10);
    let txr: Arc<Sender<Events>> = Arc::new(tx);

    let event_channel_a = EventChannel::new(txr.clone());
    let event_channel_b = EventChannel::new(txr.clone());
    let event_channel_c = EventChannel::new(txr.clone());

    //
    // Send start event to initialize sequence
    //
    let _ = event_channel_a.tx.clone().send(Start);

    init_task_a(event_channel_a);
    init_task_b(event_channel_b);
    init_task_c(event_channel_c);

    loop {
        match rx.recv().await.unwrap() {
            Start => info!("Started"),
            TaskAFinished => info!("Task A finished"),
            TaskBFinished => info!("Task B finished"),
            TaskCFinished => {
                info!("Task C finished");
                let _ = txr.send(Complete);
            }
            Complete => {
                info!("Completed");
                let _ = txr.send(Start);
                warn!("Looping forever")
                // break;
            }
        }
    }
}
