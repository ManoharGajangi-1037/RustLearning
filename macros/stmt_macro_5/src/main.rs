macro_rules! repeat_twice {
    ($s:stmt$(;)?) => {
        $s
        $s
    };
}

macro_rules! repeat_many {
    ($($s:stmt)*) => {
        $(
            $s
            // println!("{}",x);
        )*
    };
}
fn main() {
    repeat_twice!(println!("Hello"));
    repeat_many!(let x = 5; println!("helloworld"); println!("hello"));
}
