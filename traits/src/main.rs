use std::fmt::format;

trait Summary{
    fn summarize(&self)->String;
}
struct User{
    name:String,
    age:u64,
}
impl Summary for User{
    fn summarize(&self)->String {
        return format!("{} is {} years old",self.name,self.age);
    }
}
fn main() {
    let user1=User{
        age:53,
        name:"manohar".to_owned(),
    };
    println!("{}",user1.summarize());
}
