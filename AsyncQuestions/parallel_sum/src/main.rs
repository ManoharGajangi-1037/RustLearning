use std::sync::Arc;

use tokio::{
    sync::mpsc::{self, Sender},
    task::JoinError,
};

async fn task_1(numbers: Arc<Vec<i32>>, tx: Sender<i32>) {
    println!("task1");
    let mut sum = 0;
    for i in 0..=4 {
        sum += numbers.get(i).unwrap();
    }
    println!("task1 {}", sum);
    tx.send(sum).await;
}
async fn task_2(numbers: Arc<Vec<i32>>, tx: Sender<i32>) {
    let mut sum = 0;
    for i in 5..=6 {
        sum += numbers.get(i).unwrap();
    }
    tx.send(sum).await;
}
async fn task_3(numbers: Arc<Vec<i32>>, tx: Sender<i32>) {
    let mut sum = 0;
    for i in 7..=9 {
        sum += numbers.get(i).unwrap();
    }
    tx.send(sum).await;
}
#[tokio::main]
async fn main() -> Result<(), JoinError> {
    let numbers = Arc::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let (tx, mut rx) = mpsc::channel::<i32>(10);

    let task_1 = tokio::spawn(task_1(numbers.clone(), tx.clone()));
    let task_2 = tokio::spawn(task_2(Arc::clone(&numbers), tx.clone()));
    let task_3 = tokio::spawn(task_3(Arc::clone(&numbers), tx.clone()));

    task_1.await?;
    task_2.await?;
    task_3.await?;
    drop(tx);
    let mut sum = 0;
    while let Some(value) = rx.recv().await {
        println!("sum value {}", value);
        sum += value;
    }

    println!("Hello, world! {}", sum);
    Ok(())
}
