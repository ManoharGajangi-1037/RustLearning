use std::rc::Rc;
use std::cell::Cell;
struct Person{
    name:String,
    age:Cell<u8>
}

impl Person{
    fn increment(&self)->u8{
        self.age.set(self.age.get()+1);
        self.age.get()
    }
}
fn main() {
    let array = vec!["Jhon".to_string(), "Joe".to_string()];
    let rc = Rc::new(array);
    let weak = Rc::downgrade(&rc);
    drop(rc);
    // let value = weak.upgrade().unwrap();
    //println!("{:?}",value);//This is going to panic as we have dropped the value ,so we can't use further
    //Rc cant mutable
    //We can have a cell::Cell

    let person=Person{
        age:Cell::new(32),
        name:"manohar".to_string()
    };
    let incremented_age=person.increment();
    println!("{}",incremented_age);
}
