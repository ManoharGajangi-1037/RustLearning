macro_rules! my_vec {
    ($element:expr;$count:expr) => {{
        let count = $count;
        // let element = $($element)+;
        let mut vs = Vec::with_capacity(count);
        vs.extend(std::iter::repeat($element).take(count));
        //   $(vs.push($element);)+
        vs
    }};
}

fn main() {
    let array = my_vec![1;3];
    println!("{:?}", array);
    println!("Hello, world!");
}
