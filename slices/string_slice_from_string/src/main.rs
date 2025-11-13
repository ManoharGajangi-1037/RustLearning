fn main() {
    let name = String::from("Manohar Gajangi");

    let first_name = &name[0..7];
    let last_name =&name[8..15];

    println!("{first_name} {last_name}")
    
}
