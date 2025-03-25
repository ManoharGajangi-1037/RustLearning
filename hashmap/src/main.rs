use std::collections::HashMap;

fn main() {
    let mut  hashmap=HashMap::new();
    hashmap.insert(1, String::from("manohar"));
    hashmap.insert(2, String::from("harsha"));
    hashmap.insert(3, String::from("laxman"));
    let iter=hashmap.iter();
    for (val,val2) in iter{
        println!("key:{}-->Value:{}",val,val2);
    }
    println!("Hello, world! {:?}",hashmap);
}
