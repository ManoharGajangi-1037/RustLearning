pub fn borrowing_example() {
    println!("This is a borrow example");
    let s1 = String::from("manohar");
    let len = calculate_length(&s1);
    println!("the length is of the string {} is {}", s1, len);
}
pub fn calculate_length(s1: &String) -> usize {
    s1.len()
}
