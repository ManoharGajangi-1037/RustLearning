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

///Trait Bound Syntax here we can mention what particular trait we are going to have in this function
///Kind of generic syntax
fn book<T: Accomodation>(entity: &mut T, name: &str, nights: u32) {
    entity.book(name, nights);
}

///Here we have 2 types of structs that are going to implement Accomodation trait at a time
/// This can be done in another way by below mix function using trait bounds  
fn first_and_second(
    first: &mut impl Accomodation,
    second: &mut impl Accomodation,
    name: &str,
    nights: u32,
) {
    first.book(name, nights);
    second.book(name, nights);
}

///This is the syntax that can be used for multiple structs which will use traits

fn mix<T: Accomodation, U: Accomodation>(first: &mut T, second: &mut U, name: &str, nights: u32) {
    first.book(name, nights);
    second.book(name, nights);
}
fn main() {
    let mut hotel = Hotel::new("Mythri");
    // book(&mut hotel, "Harin", 6);
    // println!("{:?}", hotel);

    // let mut air_bnb = AirBnB::new("Shiva");
    // book(&mut air_bnb, "manohar", 5);
    // println!("{:?}", air_bnb);

    // mix(&mut hotel, &mut air_bnb, "hello", 6);

    let mut hotel_1 = Hotel::new("SKY");
    let mut hotel_2 = AirBnB::new("YSKY");

    let array = [1, 2, 3];
    let mut lodges: Vec<&mut dyn Accomodation> = vec![&mut hotel_1, &mut hotel_2];
    for lodge in &mut lodges {
        lodge.book("manohar", 5);
    }
    println!("{:?}", hotel_1);
}
