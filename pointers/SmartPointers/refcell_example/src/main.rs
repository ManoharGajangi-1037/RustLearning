use std::cell::RefCell;

fn main() {
    //In the below case we cannot able to mutate the following because it does not have mutable variable 
    // let y = "hello".to_string();
    // y.push_str("string");
    // println!("{}",y);

     //Here even though it has not mutable variable it is mutating and compiler not complains
    // because it has borrowchecking rules at compile time 
    // Many immutable borrows (borrow()) allowed

    // OR one mutable borrow (borrow_mut())

    // Checked at runtime; violating causes panic.
    let x = RefCell::new(5);
    let y=x.borrow_mut();
    // println!("{}",x.borrow());

    //It is only for single threaded environment 


}
