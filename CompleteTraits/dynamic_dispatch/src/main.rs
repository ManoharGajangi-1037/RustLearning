use expand::expand;

struct Dog {
    name: String,
}
struct Cat {
    name: String,
}

trait Animal {
    fn speak(&self);
}

impl Animal for Dog {
    fn speak(&self) {
        println!("{} communicates as Bow Bow", self.name);
    }
}
impl Animal for Cat {
    fn speak(&self) {
        println!("{} communicates as Meow Meow ", self.name);
    }
}

fn speak(entity: Box<dyn Animal>) {
    entity.speak();
}
fn main() {
    let cat = Box::new(Cat {
        name: "Rosy".to_string(),
    });
    expand!(println!("{:p}", cat.name.as_ptr()));
    let dog = Box::new(Dog {
        name: "snoopi".to_string(),
    });
    speak(dog);
    speak(cat);
}
