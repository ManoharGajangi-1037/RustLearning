use std::{
    fs::File,
    io::{Error, Read},
};

async fn file_read(path: &'static str) -> Result<String, Error> {
    let task = tokio::task::spawn_blocking(move || {
        let result = File::open(path.clone());
        let Ok(mut file_name) = result else {
            return Err(result.err().unwrap());
        };
        let mut contents = String::new();
        file_name.read_to_string(&mut contents)?;
        Ok(contents)
    });
    let result = task.await.unwrap();
    result
}

#[tokio::main]
async fn main() {
    let result = file_read("helloword.txt").await;
    println!("{:?}", result);
    println!("Hello, world!");
}
