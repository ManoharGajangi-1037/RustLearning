// use std::{sync::Barrier, thread};

// fn main() {
//     println!("Hello, world!");
//     let barrier = Barrier::new(4);
//     // let handle = vec![]
//     thread::scope(|s|{
//         for   _ in 0..4{
//             s.spawn(||{
//                   println!("Reached Barrier");
//                   barrier.wait();
//                   println!("Paased Barrier")
//             });
//         }
//     }
//     );
// }

use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
    let barrier = Arc::new(Barrier::new(4));
    let mut handles = vec![];

    for i in 0..4 {
        let b = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            println!("Thread {i} reached the barrier");
            b.wait();
            println!("Thread {i} passed the barrier");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
