
struct Pair<T>{
    x:T,
    y:T
}

impl<T> Pair<T>{
   fn new(x:T,y:T)->Self{
    Pair { x, y}
   }

   fn max<T:PartialOrd>(a:T,b:T)->T{
       if a > b {a} else {b}
   }
}
fn main() {
    let int_pair=Pair::new(1,1);
    let float_pair=Pair::new(1.2, 3.2);

}
