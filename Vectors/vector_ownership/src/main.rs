fn main() {
    let mut array =vec!["Hello".to_string(),"World".to_string()];

    let reference = &array[1];
    array.push("manohar".to_string());

    println!("{:?}",array);

    // println!("{reference}")
}
