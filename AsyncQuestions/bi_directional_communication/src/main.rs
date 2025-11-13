use std::thread;
use std::sync::mpsc;
fn main() {
    let (tx1,rx1)=mpsc::channel();
    let (tx2,rx2)=mpsc::channel();
    let client = thread::spawn(move ||{
        println!("clientt");
        tx1.send(5).unwrap();
        if let Ok(val) = rx2.recv() {
            println!("server sent value {}",val);
        }
    });
 
    let server = thread::spawn(move ||{
        while let Ok(value) = rx1.recv() {
            println!("client sent value {} ",value);
            tx2.send(6).unwrap();
            // drop(tx2);
        }
    });

    client.join().unwrap();
    server.join().unwrap();
}
