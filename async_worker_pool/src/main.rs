use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::{
    signal,
    sync::mpsc::{self, Receiver},
    time::{Duration, sleep},
};

#[derive(Debug)]
struct Job {
    id: usize,
    description: String,
}

async fn worker(id: usize, shared_rx: Arc<Mutex<Receiver<Job>>>) {
    loop {
        let maybe_job = {
            let mut rx = shared_rx.lock().await;
            rx.recv().await
        };

        match maybe_job {
            Some(job) => {
                println!("👷 Worker {} started job: {:?}", id, job);
                // Simulate some async work
                sleep(Duration::from_secs(2)).await;
                println!("✅ Worker {} finished job {}", id, job.id);
            }
            None => {
                println!("🛑 Worker {}: Channel closed. Exiting.", id);
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel::<Job>(100);

    // Wrap receiver in Arc<Mutex<>> so all workers can access it
    let shared_rx = Arc::new(Mutex::new(rx));

    let worker_count = 3;
    let mut handles = Vec::new();

    for id in 0..worker_count {
        let worker_rx = Arc::clone(&shared_rx);
        handles.push(tokio::spawn(worker(id, worker_rx)));
    }

    let tx_clone = tx.clone();
    tokio::spawn(async move {
        for id in 0..10 {
            tx_clone
                .send(Job {
                    id,
                    description: format!("Job {}", id),
                })
                .await
                .unwrap();
        }
        println!("📬 All jobs submitted. Press Ctrl+C to stop gracefully.");
    });
    // Wait for graceful shutdown signal
    signal::ctrl_c().await.unwrap();
    println!("🧹 Graceful shutdown initiated");

    // Drop sender to close the channel and exit all workers
    drop(tx);

    // Wait for all workers to finish
    for handle in handles {
        let _ = handle.await;
    }

    println!("🏁 All workers stopped. Shutdown complete.");
}
