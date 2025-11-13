use std::{rc, sync::mpsc::channel, thread};

fn main() {
    let (tx,rx)=channel();
    let mut handles = vec![];
    // let value = tx.clone();
    // value.send(5);
    for i in 0..3{
        let value= tx.clone();
        let handle = thread::spawn(move ||{
            value.send(i);
            // drop(value);
        });
        handles.push(handle);
    }
    drop(tx);
    while let Ok(value) = rx.recv(){
        println!("value :{}",value);
    }
    // for handle in handles{
    //     handle.join().unwrap();
    // }
    println!("Hello, world!");
}
