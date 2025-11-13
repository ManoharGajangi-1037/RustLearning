use std::time::Duration;

use tokio::{sync::oneshot, task, time::sleep};

async fn wait_and_exit() {
    let (tx, rx) = oneshot::channel::<i32>();
    let task = task::spawn_blocking(async || {
        sleep(Duration::from_secs(3)).await;
        tx.send(5);
    });

    let timer = sleep(Duration::from_secs(2));

    tokio::select! {
        result = rx =>{
            match result{
                 Ok(msg) => println!("Got message first {}",msg),
                 Err(_)=>println!("some error occured")
            }
        },
        _ = timer =>{
             println!("timer arrived")
        }
    }
}
#[tokio::main]
async fn main() {
    wait_and_exit().await;
    println!("Hello, world!");
}
