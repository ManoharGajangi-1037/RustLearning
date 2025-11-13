use std::collections::HashMap;

trait Accomodation {
    fn get_description(&self) -> String;
    fn book(&mut self, name: &str, nights: u32);
}

#[derive(Debug)]
struct Hotel {
    name: String,
    reservations: HashMap<String, u32>,
}

impl Hotel {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            reservations: HashMap::new(),
        }
    }
}

//Implementing Accomodation trait for Hotel
impl Accomodation for Hotel {
    fn get_description(&self) -> String {
        format!("Hotel::{}", &self.name)
    }
    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

#[derive(Debug)]
struct AirBnB {
    name: String,
    guests: Vec<(String, u32)>,
}

impl AirBnB {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            guests: Vec::new(),
        }
    }
}
///Implementing Accomodation trai for AirBnB
impl Accomodation for AirBnB {
    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights));
    }
    fn get_description(&self) -> String {
        format!("Air BnB name {}", &self.name)
    }
}

fn book(entity: &mut dyn Accomodation) -> String {
    let result = entity.get_description();
    result
}
fn main() {
    let mut hotel = Hotel::new("Mythri");
    println!("{}", hotel.get_description());
    hotel.book("manohar", 5);
    println!("{:?}", hotel);

    let mut air_bnb = AirBnB::new("Shiva");

    // println!("{}", air_bnb.get_description());
    // air_bnb.book("manohar", 5);
    // println!("{:?}", air_bnb);
    let array: &mut dyn Accomodation = [&mut hotel, &mut air_bnb];
    // let result = book(&mut hotel);
    // let result_2 = book(&mut air_bnb);
    // println!("{:?}", result);
}
