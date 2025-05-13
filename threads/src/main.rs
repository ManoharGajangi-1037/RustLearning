use std::thread;
use std::time::Duration;
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10000 {
            println!("printing from thread {}", i);
        }
    });
    handle.join().unwrap();
    for i in 1..5000 {
        println!("printing from main {}", i);
    }
}
