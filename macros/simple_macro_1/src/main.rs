macro_rules! hello {
    ($name:expr) => {
        println!("Hello world {}", ($name) + ($name));
    };
    () => {
        println!("hii");
    };
}

macro_rules! make_fn {
    ($name:ident) => {
        fn $name() {
            println!("Hello World");
        }
    };
}

macro_rules! make_vec_type {
    ($name:ty) => {
        Vec<$name>
    };
}
make_fn!(run);

macro_rules! my_vec {
    //  Here $(____), This imples we are having somedata followed by , and + indicates 1 or more
    ($($element:expr),+) => {
        {
            let mut vs= Vec::new();
            $(vs.push($element);)*;
            vs
        }
    };
    ($($element:expr);+;/$count:expr)=>{
        {
            println!("Hello New Vec");
            let mut vs= Vec::new();
            let mut i = 0 ;
            while i < $count{
                $(
                if i == $count {
                    break;
                }
                vs.push($element);
                i+=1;
                )+
            }
            vs
        }
    };
}
fn main() {
    println!("Hey Let's learn about macros");

    /*
    The macro println! will be converting as below
        {
            ::std::io::_print(format_args!("Hey Let\'s learn about macros\n"));
        };
    */
    //This macro will try to match empty () handler and will give respective output
    // hello!();

    // This macro will be taking  2 * 5  = 10 as input ,we can print it or we can use it for further
    // hello!(2 * 5);

    //make_fn macro will create a function ,just we can call it .
    run();

    //This macro is defined for declaring types
    let sample_vec: make_vec_type!(i32) = vec![1, 2, 3];
    // println!("{:?}", sample_vec);

    //This macro is creating a vector
    let array = my_vec![1;2;3;/4];

    println!("{:?}", array);
}
