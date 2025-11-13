#[derive(Debug)]
struct Coffee{
    name: String,
    price: f64,
    is_hot: bool,
}

impl Coffee{

    fn hello(name:String,price:f64,is_hot:bool)->Self{
        Coffee { name, price, is_hot}

    }
    //Here we can access mutable ,immutable with reference or without depending upon our use case
    fn display_coffee(&self){
        println!("{} -- {} -- {}",&self.name,&self.price,&self.is_hot)
    }

    fn change_coffee_name(&mut self){
        self.name = "Chai".to_string();
    }
}
fn main() {
    let mut new_coffee = Coffee{
        name:"Mocha".to_string(),
        price:77.7,
        is_hot:true,
    };

    let name = "chai ".to_string();
    let price = 3.6;
    let is_hot=true;
    let coffee= Coffee::hello(name, price, is_hot);
    new_coffee.change_coffee_name();
    new_coffee.display_coffee();

    // let mut name = "Manohar".to_string();
    // let mut one_more = &mut name;
    // one_more.push_str("hello");
    // println!("{}",name);

    println!("{:?}",new_coffee);
}
