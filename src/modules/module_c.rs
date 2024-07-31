use crate::common::events::{EventChannel, Events};
use std::time::Duration;
use tokio::time::sleep;

async fn task_c(mut channel: EventChannel) {
    loop {
        match channel.rx.recv().await.unwrap() {
            Events::TaskBFinished => {
                info!("Task C runnnig");
                sleep(Duration::from_secs(1)).await;
                let _ = channel.tx.send(Events::TaskCFinished);
            }
            _ => {
                sleep(Duration::from_millis(1)).await;
            }
        }
    }
}

pub fn init_task_c(channel: EventChannel) -> tokio::task::JoinHandle<()> {
    tokio::spawn(task_c(channel))
}
