use std::time::Duration;

use rand::random_range;
use tokio::time::sleep;
async fn task() {
    let range = random_range(0..3);
    sleep(Duration::from_secs(range)).await
}
#[tokio::main]
async fn main() {
    let task_2 = sleep(Duration::from_secs(2));

    let result = tokio::time::timeout(Duration::from_secs(2), task()).await;

    match result {
        Ok(_) => println!("task completed"),
        Err(_) => println!("timeout"),
    }
    // tokio::select! {
    //     _ = task()=>{
    //           println!("task completed first")
    //     },
    //     _ =task_2=>{
    //         println!("sleep completed first")
    //     }
    // };
}
