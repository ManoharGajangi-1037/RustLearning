fn main() {
    println!("Hello, world!");
    let array = ["hello".to_string(), "world".to_string(), "hyder".to_string()];
    
    let mut first = "Hello".to_string();
    let third=&mut first;
    // let second = &first;
    println!("{third}");
}