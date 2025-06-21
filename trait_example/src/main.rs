use std::fmt::format;

trait Summary{
    fn detailing(){
        println!("detailing is here");
    }
    fn summary(&self)-> String;
}
struct User{
    id:u32,
    name:String
}
impl Summary for User{
   fn summary(&self)-> String {
       format!("{} holding name {}",self.id,self.name)
   }
}
fn main() {
    let user=User{
        id:1,
        name:"Manohar".to_string()
    };
    println!("Hello, world!{}",user.summary());
}
