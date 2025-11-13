use std::collections::HashMap;

use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct User {
    email: String,

    //If all other field types are unknown but you want to tract them
    //It converts automatically and gives the data
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}
#[tokio::main]
async fn main() {
    let url = "https://jsonplaceholder.typicode.com/users";
    let client = Client::new();

    let result: Vec<User> = client.get(url).send().await.unwrap().json().await.unwrap();

    println!("{:#?}", result.get(0).unwrap().email);
    // println!(
    //     "{}",
    //     result
    //         .get(0)
    //         .and_then(|u| u.get("email"))
    //         .and_then(|v| v.as_str())
    //         .unwrap_or("unknown")
    // );
}
