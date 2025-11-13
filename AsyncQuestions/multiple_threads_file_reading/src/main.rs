use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Read},
    sync::mpsc::{self, SendError, Sender},
    thread, vec,
};
#[derive(Debug)]
enum FileReadError {
    Io(Error),
    Send(SendError<(String, Result<String, Error>)>),
}

use tokio::task;

fn file_read(
    path: &str,
    tx_clone: Sender<(String, Result<String, Error>)>,
) -> Result<(), FileReadError> {
    let mut file = File::open(path).map_err(|err| FileReadError::Io(err))?;

    let mut reader = BufReader::new(file);
    for line in reader.lines() {
        tx_clone
            .send((path.to_string(), line))
            .map_err(|err| FileReadError::Send(err))?;
    }
    Ok(())
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)?;
    // Ok((contents))
}
#[tokio::main]
async fn main() -> Result<(), FileReadError> {
    let path = ["file1.txt", "file2.txt", "file3.txt"];
    let (tx, rx) = mpsc::channel();

    let writer_handle = thread::spawn(|| {
        for (path, result) in rx {
            match result {
                Ok(value) => println!("{}-->{}", path, value),
                Err(e) => println!("-->{}", e),
            }
        }
    });

    let task = task::spawn_blocking(move || -> Result<(), FileReadError> {
        let mut handles = vec![];
        for path in path {
            let tx_clone = tx.clone();
            let handle = thread::spawn(
                move || file_read(path, tx_clone), // tx_clone.send((path.to_string(), result)).unwrap();
            );
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap()?;
        }
        drop(tx);
        Ok(())
        // Ok::<(), FileReadError>(())
    });
    task.await.unwrap()?;

    writer_handle.join().unwrap();
    Ok(())
}
