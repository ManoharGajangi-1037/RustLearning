fn main() {
    let first_name = {
        let full_name = "Manohar Gajangi";
        &full_name[0..8]
    };
    println!("{first_name}");

    let mut name = "Hello";
    let second_name =&mut name;
}
