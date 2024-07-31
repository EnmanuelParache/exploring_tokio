use crate::common::events::{EventChannel, Events};
use std::time::Duration;
use tokio::time::sleep;

async fn task_b(mut channel: EventChannel) {
    loop {
        match channel.rx.recv().await.unwrap() {
            Events::TaskAFinished => {
                info!("Task B running");
                sleep(Duration::from_secs(1)).await;
                let _ = channel.tx.send(Events::TaskBFinished);
            }
            _ => {
                sleep(Duration::from_millis(1)).await;
            }
        }
    }
}

pub fn init_task_b(channel: EventChannel) -> tokio::task::JoinHandle<()> {
    tokio::spawn(task_b(channel))
}
