use tokio::{
    sync::mpsc,
    time::{sleep, Duration},
    signal,
};
use std::sync::Arc;

#[derive(Debug)]
struct Task {
    name: String,
    delay_secs: u64,
}

async fn task_runner(task: Task) {
    println!("🕒 Scheduled: {}", task.name);
    sleep(Duration::from_secs(task.delay_secs)).await;
    println!("✅ Done: {}", task.name);
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<Task>(100);

    // Spawn the scheduler
    tokio::spawn(async move {
        while let Some(task) = rx.recv().await {
            tokio::spawn(task_runner(task));
        }
    });

    // Simulate new task input every 2 seconds
    let tx_clone = tx.clone();
    tokio::spawn(async move{
        let mut counter = 1;
        loop {
            sleep(Duration::from_secs(2)).await;
            let _ = tx_clone.send(Task {
                name: format!("Task {}", counter),
                delay_secs: 5,
            }).await;
            counter += 1;
        }
    });

    println!("⏳ Scheduler running... Press Ctrl+C to exit");

    // Graceful shutdown
    signal::ctrl_c().await.unwrap();
    println!("👋 Shutting down gracefully.");
}
