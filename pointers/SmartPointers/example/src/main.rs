use std::{cell::Cell, rc::Rc};

fn main() {
     let x= 5;
     let b = Box::new(x);
    println!("----{}",b);
    // let mut reference= Cell::new(x);
    // let ans = reference.get()+1;
    let x=5;


    //Rc::new() use case 
    //Here we can't able to use multiple owners at a time 
    let x="manohar".to_string();
    let b=x;
    // drop(x);
    println!("{b}");

    //Here we can use multiple owners by help of Rc::clone()
    let mut reference_2= Rc::new(5);
    let a= Rc::clone(&reference_2);
    let b= Rc::clone(&a);
    drop(reference_2);
    drop(a);
    println!("{:?}",b);


    
}
