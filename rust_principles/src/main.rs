use rust_principles::borrow;
pub fn ownership_example(){
    let s1=String::from("manohar");
    let s2=s1;
    println!("the owner now is {}",s2);
    // println!("the owner now is {}",s1); //s1 doesn't prints due to transfer of owner
}

fn main() {
    ownership_example();
    borrow::borrowing_example();
}
