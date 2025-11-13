fn main() {
    println!("Hello, world!");
    let mut name = String::from("Harkirat");
    name.push_str(" Singh");
    println!("name is {}", name);
    name.replace_range(8..9, "");
    println!("name is {}", name);

    let airline = "United Airlines Flight";
    let segments = airline.split(" ").collect::<Vec<&str>>();

    let mut s1 = String::from("Hello World");
    // let s3 = &mut s1;
    let s4 = &mut s1;
    s4.push_str("hii");
    // let s2 = &s1;
    println!("{}", s4)
}
