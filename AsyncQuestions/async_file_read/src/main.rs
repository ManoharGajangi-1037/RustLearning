use std::io::{self, Error, read_to_string};

use tokio::{
    fs::{File, read},
    io::{AsyncBufReadExt, BufReader},
};

async fn file_read(path: &str) -> Result<(), std::io::Error> {
    let file = File::open(path).await?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        println!("{}", line);
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let file_read1 = file_read("helloword.txt");
    let file_read2 = file_read("practice.txt");
    let task_1 = tokio::spawn(file_read1);
    let task_2 = tokio::spawn(file_read2);
    let _result = tokio::join!(task_1, task_2);
    // println!("{:?}", file_read1);
    // println!("{:?}", file_read2);
}
