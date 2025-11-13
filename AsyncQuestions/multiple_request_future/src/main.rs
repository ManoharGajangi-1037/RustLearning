use futures::future::join_all;
use reqwest::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let urls = vec![
        "https://jsonplaceholder.typicode.com/users/1",
        "https://jsonplaceholder.typicode.com/users/2",
        "https://jsonplaceholder.typicode.com/users/3",
    ];
    let futs = urls.iter().map(|x| fetch_content(&client, x));
    let results = join_all(futs).await;
    for result in results {
        match result {
            Ok(value) => println!("{}", value),
            Err(e) => println!("{}", e),
        }
    }
    println!("{:?}", urls);
    println!("Hello, world!");
}

async fn fetch_content(client: &Client, url: &str) -> anyhow::Result<String> {
    let result = client.get(url).send().await?.text().await?;
    Ok(result)
}
