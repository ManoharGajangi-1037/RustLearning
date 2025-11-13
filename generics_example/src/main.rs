use std::cmp::Ordering;


struct Pair<T>{
    x:T,
    y:T
}

impl<T> Pair<T>{
   fn new(x:T,y:T)->Self{
    Pair { x, y}
   }
}
fn max<T:PartialOrd>(x:T,y:T) -> T {
        if x >= y { x } else { y }
}

impl<T:PartialEq> PartialEq for Pair<T>{
   fn eq(&self, other: &Self) -> bool {
       self.x == other.x && self.y == other.y
   }
}
impl<T:PartialOrd> PartialOrd for Pair<T>{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.x.partial_cmp(&other.x){
            Some(Ordering::Equal)=>self.y.partial_cmp(&other.y),
            non_eq=>non_eq
        }
    }
}

// You write p1 < p2
//          │
//          ▼
// Rust calls PartialOrd::lt
//          │
//          ▼
// lt calls p1.partial_cmp(p2)
//          │
//          ▼
// Your logic returns Option<Ordering> (Less/Equal/Greater/None)
//          │
//          ▼
// lt converts Ordering → bool
//          │
//          ▼
// You get true or false

fn main() {
    let int_pair=Pair::new(1,1);
    let second_pair = Pair::new(2, 3);
    println!("---{}",int_pair<second_pair);
    let float_pair=Pair::new(1.2, 3.2);
    let int_result = max(int_pair.x, int_pair.y);
    let float_result=max(float_pair.x, float_pair.y);
}
