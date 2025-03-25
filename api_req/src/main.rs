use reqwest;
use tokio;
#[tokio::main]
async fn main() {
    // let url = "https://official-joke-api.appspot.com/jokes/programming/random";
    let url="iinas";
    match reqwest::get(url).await {
        Ok(response) => match response.text().await {
            Ok(text) => print!("The data is {}", text),
            Err(err) => print!("Error is {}", err),
        },
        Err(err) => println!("Error while fetching data{}",err),
    }
}
