use reqwest::Client;
use tokio::task;

type MyResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
#[tokio::main]
async fn main() {
    let client = Client::new();
    let urls = vec![
        "https://jsonplaceholder.typicode.com/users/1",
        "https://jsonplaceholder.typicode.com/users/2",
        "https://jsonplaceholder.typicode.com/users/3",
    ];

    let mut handles = vec![];

    for url in urls {
        let client = client.clone();
        let handle = task::spawn(async move { fetch_text(&client, url).await });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.await.unwrap();
        match result {
            Ok(Value) => println!("{}", Value),
            Err(_) => println!("error occured"),
        }
    }
}

async fn fetch_text(client: &Client, url: &str) -> anyhow::Result<String> {
    let result = client.get(url).send().await?.text().await?;
    Ok(result)
}
