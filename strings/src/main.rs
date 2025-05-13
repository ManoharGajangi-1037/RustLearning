fn main() {
    println!("Hello, world!");
    let mut name=String::from("Harkirat");
    name.push_str(" Singh");
    println!("name is {}",name);
    name.replace_range(8..9,"" );
    println!("name is {}",name)
}
