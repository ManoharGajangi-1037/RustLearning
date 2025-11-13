fn some_function<'a>(a: &'a str, b: &'a str) -> &'a str {
    println!("{}", a);
    b
}
fn main() {
    println!("Hello, world!");

    let a = 5;
    let s1 = String::from("manohar");
    let s2 = String::from("manohar");

    some_function(&s1, &s2);
    {
        let b = 5;
    }
}
