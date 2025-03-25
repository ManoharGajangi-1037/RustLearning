fn main() {
   let mut vec=Vec::new();
   vec.push(1);
   vec.push(2);
   vec.push(3);
   vec.push(4);
   print!("{:?}",even_filter(vec));
}

fn even_filter(vec:Vec<i32>)->Vec<i32>{
    let mut even_vec=Vec::new();
    for val in vec{
        if val % 2 ==0 {
            even_vec.push(val);
        }
    }
    even_vec
}