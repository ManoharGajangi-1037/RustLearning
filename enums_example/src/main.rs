// #[derive(Debug)]
enum Shape {
    Rectangle(f64,f64),
    Circle(f64),
}
pub fn area(shape:Shape)->f64 {
    match shape{
        Shape::Rectangle(length,breadth)=>length*breadth,
        Shape::Circle(radius)=>std::f64::consts::PI*radius*radius
    }
}
fn main() {
    let a=Shape::Rectangle(4.0, 2.0);
    let b=Shape::Circle(2.0);
    println!("Hello, world!{:?}",area(b));
}
