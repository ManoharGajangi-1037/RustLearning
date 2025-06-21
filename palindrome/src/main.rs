fn main() {
    let str = String::from("madam");
    let len = str.len();
    let mut i = 0;
    let mut j = len - 1;
    while i < j {
        if str.chars().nth(i) != str.chars().nth(j) {
            println!("Not a Palindrome");
            return;
        }
        i=i+1;
        j=j-1;
    }
    for val in str.chars(){
        print!("{}",val);
    }
    println!("Palindrome");
}
