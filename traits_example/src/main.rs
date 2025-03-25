trait Speak {
    fn speak(&self);
}
struct Dog;
impl Speak for Dog{
    fn speak(&self){
       print!("Dog is barking ")
    }
}
fn main() {
    let dog_sound=Dog;
   dog_sound.speak();
  
}
