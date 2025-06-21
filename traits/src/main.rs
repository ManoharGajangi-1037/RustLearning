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
// trait HasFullName{
//     fn full_name(&self)->String;
// }
// impl HasFullName for User{
//     fn full_name(&self)->String {
//         format!("{}",self.name)
//     }
// }
trait CanRun{
    fn run(&self);
}

impl CanRun for User{
    fn run(&self) {
        
    }
}

//Trait Bounds
// fn print_details<T:HasFullName>(value:&T){
//     print!("{}",value.full_name());
// }

// fn print_details<T:HasFullName+CanRun>(value:&T){
//     print!("{}",value.full_name());
//     print!("{:?}",value.run());
// }
//We can write the same thing above as below 
fn print_details<T>(value:&T) where T : HasFullName + CanRun{
    print!("{}",value.full_name());
    print!("{:?}",value.run());
}
fn print_full_name(value:&impl HasFullName){
    println!("{}",value.full_name());
}

//TO DO LATER 
// //If we implement 

trait HasName{
    fn first_name(&self)->String;
    fn last_name(&self)->String;
}
trait HasFullName where Self:HasName{
    fn full_name(&self)->String;
}

impl<T> HasFullName for T where T:HasName{
    fn full_name(&self)->String {
        format!("{} {}",self.first_name(),self.last_name())
    }
}
struct Person{
    first_name:String,
    last_name:String
}

impl HasName for Person{
     fn first_name(&self)->String {
         self.first_name.clone()
     }
     fn last_name(&self)->String {
         self.last_name.clone()
     }
}
//If HasName is implemented for a person struct which consists of first and last name then even though we not implements HasFullname for person we can able to print full_name 


fn main() {
    let user1=User{
        age:53,
        name:"manohar".to_string(),
    };
    println!("{}",user1.summarize());
    let person=Person{
        first_name:"Manohar".to_string(),
        last_name:"Gajangi".to_string()
    };
    println!("{}",person.full_name())
    // print_full_name(&user1);
}
