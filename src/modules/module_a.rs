use crate::common::events::{EventChannel, Events};
use std::time::Duration;
use tokio::time::sleep;

async fn task_a(mut channel: EventChannel) {
    loop {
        match channel.rx.recv().await.unwrap() {
            Events::Start => {
                info!("Task A running");
                sleep(Duration::from_secs(1)).await;
                let _ = channel.tx.send(Events::TaskAFinished);
            }
            _ => {
                sleep(Duration::from_millis(1)).await;
            }
        }
    }
}

pub fn init_task_a(channel: EventChannel) -> tokio::task::JoinHandle<()> {
    tokio::spawn(task_a(channel))
}
