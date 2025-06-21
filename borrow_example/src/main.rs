use std::fmt::format;

trait Summary {
    fn summarize(&self)->String;
}

struct User{
    name:String,
    age:i32,
}

impl Summary for User{
    fn summarize(&self)->String {
        format!("{} has {}",self.name,self.age)
    }
}

fn main() {
    let user = User{
        age:32,
        name:"Manohar".to_string()
    };
    print!("{}",user.summarize());
}
