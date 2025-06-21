use std::collections::HashMap;

fn count_words(input :&str)->HashMap<String,usize>{
    let mut hm=HashMap::new();
    for word in input.split_whitespace(){
        *hm.entry(word.to_string()).or_insert(0)+=1;
    }
    hm
}
fn main() {
    let counts=count_words("Apple Bat Apple Bat Cat Dog");
    for (word ,count) in counts{
        println!("{}-->{}",word,count);
    }
}
