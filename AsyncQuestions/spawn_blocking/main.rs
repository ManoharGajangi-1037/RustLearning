use std::io::Error;

use tokio::{
    fs::File,
    io::{self, AsyncReadExt},
    task::JoinSet,
};

async fn file_read(path: &str) -> Result<(String, String), Error> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await;
    Ok((path.to_string(), contents))
}
#[tokio::main]
async fn main() {
    let mut handle = JoinSet::new();
    handle.spawn(file_read("helloword.txt"));
    handle.spawn(file_read("notes.txt"));
    handle.spawn(file_read("practice.txt"));

    let mut results = Vec::new();

    while let Some(res) = handle.join_next().await {
        match res {
            Ok(Ok((file, contents))) => results.push((file, contents)),
            Ok(Err(e)) => eprintln!("error occured while reading file "),
            Err(e) => eprintln!("Task error"),
        }
    }

    for (path, contents) in results {
        println!("File name --> {} Its contents:\n{}", path, contents);
    }
}
