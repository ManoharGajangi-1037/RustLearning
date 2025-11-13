use std::{collections::HashMap};

trait Accomodation {
    fn get_description(&self) -> String;
    fn book(&mut self,name:&str,nights:u32);
}

#[derive(Debug)]
struct Hotel{
    name: String,
    reservations: HashMap<String,u32>,
}

impl Hotel{
    fn new(name: &str) -> Self{
        Self { name: name.to_string(), reservations:HashMap::new()}
    }


    //Here we ara able to call the trait method from out struct 
    fn summarize(&self) -> String {
       format!("{} --- {}",self.name,self.get_description())
    }
}


//Implementing Accomodation trait for Hotel 
impl Accomodation for Hotel{
    fn get_description(&self) -> String {
        format!("Hotel::{}",&self.name)
    }
    fn book(&mut self,name:&str,nights:u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

#[derive(Debug)]
struct AirBnB{
    name: String,
    guests: Vec<(String,u32)>
}

impl AirBnB{
    fn new(name: &str) -> Self{
        Self { name: name.to_string(), guests: Vec::new() }
    }
}
///Implementing Accomodation trai for AirBnB
impl Accomodation for AirBnB{
    fn book(&mut self,name:&str,nights:u32) {
        self.guests.push((name.to_string(),nights));
    }
    fn get_description(&self) -> String {
        format!("Air BnB name {}",&self.name)
    }
}
fn main() {
    let mut hotel = Hotel::new("Mythri");
    println!("{}",hotel.summarize());
    hotel.book("manohar", 5);
    println!("{:?}",hotel);

    let mut air_bnb = AirBnB::new("Shiva");

    println!("{}",air_bnb.get_description());
    air_bnb.book("manohar", 5);
    println!("{:?}",air_bnb);
}
