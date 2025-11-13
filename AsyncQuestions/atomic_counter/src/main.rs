use std::{sync::{Arc, atomic::AtomicI32}, thread};

fn main() {

    let counter =   Arc::new(AtomicI32::new(0));
    let mut handles = vec![];
    for i in 0..5{
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move ||{
             for j in 0..1000{
                counter_clone.fetch_add(1,std::sync::atomic::Ordering::Relaxed);
             }
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }
    println!("Hello, world! {:?}",counter);
}
