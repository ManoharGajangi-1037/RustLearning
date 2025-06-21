enum Operation{
    Add,
    Subtract,
    Divide,
    Multiply
}

fn calculate(op:Operation,a:f64,b:f64)->Option<f64>{
    match op{
        Operation::Add=> Some(a+b),
        Operation::Divide=> Some(a / b),
        Operation::Multiply=> Some(a * b),
        Operation::Subtract=> Some(a - b),
    }
}
fn main() {
  let result= calculate(Operation::Divide, 1.0, 2.0);
     let ans=match result {
         Some(x)=>  x,
         None=> 0.0
     };
}
