use futures::executor::block_on;
use tokio::time::{sleep,Duration};

async fn get_name()->String{
    "john".to_string()
}

async fn call_api_one()->String{
    sleep(Duration::from_secs(1)).await;
    "One".to_string()
}


async fn call_api_two()->String{
    sleep(Duration::from_secs(1)).await;
    "Two".to_string()
}
#[tokio::main]
async fn main() {
    let name=block_on(get_name());//related to futures crate
    let one=call_api_one().await;
    let two=call_api_two().await;
    println!("{}",one);     
     println!("{}",two);     
}
