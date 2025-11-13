use std::{sync::mpsc::channel, thread};

use crossbeam::channel;
fn main() {

    //we cannot clone the rx1 as it is from std::mpsc 
    let (tx1,rx1) = std::sync::mpsc::channel::<i32>(); 
    

    //we can clone rx as it can recieve multiple (any one of them will recieve the data)
    let (tx,rx) =  channel::unbounded::<i32>();
    let sending = thread::spawn(move ||{

        for i in 0..5{
            tx.send(i);
        }
    });

    let crossbeam_rec1 = rx.clone();
    let crossbeam_rec2 = rx.clone();


    let reciever_1 = thread::spawn(move ||{
        while let Ok(val) = crossbeam_rec1.recv(){
            println!("reciever 1 got --> {}",val);
        }
    });

    let reciever_2= thread::spawn(move||{
         while let Ok(val) = crossbeam_rec2.recv(){
            println!("reciever 2 got --> {}",val);
        }
    });
    sending.join().unwrap();
    reciever_1.join().unwrap();
    reciever_2.join().unwrap();
    println!("Hello, world!");
}
