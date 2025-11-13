macro_rules! count_idents {
    () => {
        0
    };
    ($head:literal $(,)? $($tail:literal),* $(,)?) => {
        //  println!("---->{}",stringify!($($tail)*));
          1+count_idents!($($tail),*);
    };
}

fn f1() {
    let count = count_idents!(1, 2, 3, 4,);
    println!("--->{}", count);
}

macro_rules! sum {
    ($x:expr) => {
        $x
    };
    ($x:expr,$($tail:expr),* ) => {
        // println!("{}", stringify!($($tail),*));
         $x + sum!($($tail),*)
    };
}
fn f2() {
    let sum = sum!(1.2, 2.0, 3.0);
    println!("sum of integers {}", sum);
}

macro_rules! fib {
    ($n:expr) => {{
        const fn cfb(n: u32) -> u32 {
            match n {
                0 => 0,
                1 => 1,
                _ => cfb(n - 1) + cfb(n - 2),
            }
        };
        cfb($n)
    }};
}

macro_rules! fib_macro{
    (0) => { 0 };
    (1) => { 1 };
    ($n:tt) => {{ // Using $n:tt to accept any token tree, often used for integer literals
        const N: u32 = $n;
        const RESULT: u32 = fib!(N - 1) + fib!(N - 2); // This will only work if N-1 and N-2 are also compile-time literals
        RESULT
    }};
}
fn f3() {
    //If you are using expr then you need to use const fun as it can't able to parse well
    let nth_fib = fib!(4);
    let x = 4;
    //Where as If we use tt then it can able to parse well
    let nth_fib_2 = fib_macro!(5);
    // println!("{}", nth_fib_2);
}
fn main() {
    f1();
    f2();
    f3();
}
