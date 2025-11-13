#[derive(Debug)]
enum CheeseSteak<T>{
    Plain,
    Topping(T)
}

fn identity<T>(value: T)->T{
     value
}

fn two_params<T,U>(value_1:T,value_2:U)->(T,U){
    (value_1,value_2)
}

#[derive(Debug)]
struct Coffee<T>{
    name:T,
    price:f64,
}

//When we mention derive for the coffe ,it will not implement debug trait for the T type 
impl<T:std::fmt::Debug> Coffee<T>{
     fn name_of_coffee(&self){
        println!("coffee name is {:?}",self.name);
     }
}
fn main() {
    println!("{}",identity(32));
    println!("{}",identity("Hello World"));
    let answer = two_params("hellow",32);
    println!("{}--{}",answer.0,answer.1);
    let mut  sample:CheeseSteak<i32> = CheeseSteak::Plain;
    sample = CheeseSteak::Topping(32);
    println!("{sample:?}");

    let coffee = Coffee{
        name:"hello",
        price:56.3,
    };

    println!("{:?}",coffee);
    coffee.name_of_coffee();

}
