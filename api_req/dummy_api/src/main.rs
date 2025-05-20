
use reqwest::Error;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}
#[derive(Debug, Deserialize)]
struct Product {
    id: u32,
    title: String,
    description: String,
    price: f64,
    discountPercentage: f64,
    rating: f64,
    stock: u32,
    brand: String,
    category: String,
    thumbnail: String,
    images: Vec<String>,
    availabilityStatus:String,
    dimensions:Dimensions
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://dummyjson.com/products/1";
    let response = reqwest::get(url).await?;
    let ans:Product=response.json().await?;
    println!("{:?}",ans);
    Ok(())
}
