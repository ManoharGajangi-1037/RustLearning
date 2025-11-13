mod helper;

macro_rules! call {
    ($func:ident) => {
        $func()
    };
}
fn greet() {
    println!("Hello World");
}
macro_rules! caller {
    ($func:path) => {
        $func()
    };
}
fn main() {
    call!(greet);
    caller!(crate::helper::greeting);
}
