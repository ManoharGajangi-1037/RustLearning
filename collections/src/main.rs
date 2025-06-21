use std::{collections::HashMap, vec};

fn get_tuple_values() -> (&'static str, &'static str, i32) {
    ("Hello", "World", 10)
}
fn main() {
    //Tuples
    let (_, _, _) = get_tuple_values();

    //Vectors
    let values: [i32; 2] = [1, 2];
    let doubled: Vec<i32> = values.iter().map(|x| x * 5).collect();
    println!("{:?}", doubled);

    //HashMaps
    let mut hashmap: HashMap<&str, String> = HashMap::new();
    hashmap.insert("Jhon", "Doe".to_string());
    //Inserts the default empty string if key not existed
    let value = hashmap.entry("Jhn").or_default();
    value.push_str("smith");

    for (key, value) in &hashmap {
        println!("{}->{}", key, value);
    }
    print!("{:?}", hashmap);

    let nums = vec![1, 2, 3, 5];
    let iter = nums.iter();
    let sum: i32 = iter.sum();
    //Iterators are lazy in rust
    // let sum2:i32=iter.sum();//You can't do this way as the value is going to be moved

    let nums2 = vec![1, 2, 3, 4, 5];
    //Here you need to collect the map ,else it won't reflect
    let _ans: Vec<i32> = nums2.iter().map(|x| x * 2).collect();

    let names = vec!["Hello", "mANOHAR", "Harsha"];
    
    /*  for value in names.iter(){
        println!("{}",value);
     }
     print!("{:?}",names);
     Here you can  use names again as the method is iter()
    */

    /*  for value in names.into_iter(){
        println!("{}",value);
     }
     print!("{:?}",names);
     Here you can't able to use the names again as the values are moved
    */
}
