use std::collections::HashMap;
fn main() {
    let mut count = HashMap::new();
    let words = vec!["Hello", "Hii", "Hello", "Hii", "Hii"];
    for word in words {
        match count.get(word) {
            Some(val) => count.insert(word, val + 1),
            None => count.insert(word, 1),
        };
    }
    let val = match count.get("Hii") {
        Some(val) => val,
        None => {
            print!("not found");
            &0
        }
    };
    println!("Hello, world!,{}",val);
}
