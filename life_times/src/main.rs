
// fn get_name()->&str{
//     "John"
// }
//Here John is created in heap and after the function ends it deallocated memory ,
//Inorder to work it out ,we have to give a lifetime access ,until its validity ,here 
//we needed it for entire program to live on so we have to mention static before that 

fn get_name()->&'static str{
    "John"
}
fn get_random_name<'a>(a:&'a str,b:&'a str)->&'a str{
 //here we might return a or b so we are having lifetimes as a for both and mentioning 'a for &str 
    a
}


fn get_full_name(fullname:&str)->&str{
    //Here the Rust compiler automatically mentioning lifetimes
    fullname
}


//3 LIfetime Rules
//#1 Compiler assigns life time for every parameter that is a reference

// fn print_refs(x: &i32, y: &i32) {
//     println!("x: {}, y: {}", x, y);
// }

//Behind the Scenes
// fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) { ... }


//#2 single input life time is assigned to output 
// fn get_first(s: &str) -> &str {
//     &s[0..1]
// }
//Rust Assumes
// fn get_first<'a>(s: &'a str) -> &'a str


//#3 if &self or &mut self in parameters ,life time of self is assigned 

// struct Person {
//     name: String,
// }

// impl Person {
//     fn get_name(&self) -> &str {
//         &self.name
//     }
// }

//Rust Assumes
// fn get_name<'a>(&'a self) -> &'a str

fn main() {
    let name=get_name();
    let random_name=get_random_name("hello","hii");
}
