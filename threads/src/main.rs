use std::thread;
use std::time::Duration;

fn main() {
    // Spawn the first thread
    let handle1 = thread::spawn(|| {
        for i in 1..=5 {
            println!("Thread 1: Count {}", i);
            thread::sleep(Duration::from_millis(500)); // Simulate work
        }
    });

    // Spawn the second thread
    let handle2 = thread::spawn(|| {
        for i in 1..=5 {
            println!("Thread 2: Count {}", i);
            thread::sleep(Duration::from_millis(500)); // Simulate work
        }
    });

    // Ensure both threads finish before the main thread exits
    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Both threads are done!");
}

