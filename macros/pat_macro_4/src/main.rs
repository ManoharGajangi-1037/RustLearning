macro_rules! test_match {
    // with a guard
    ($pattern:pat if $guard:expr, $val:expr) => {
        match $val {
            $pattern if $guard => println!("hello value is {:?}", $val),
            _ => println!("not matched"),
        }
    };
    // without a guard
    ($pattern:pat, $val:expr) => {
        match $val {
            $pattern => println!("hello value is {:?}", $val),
            _ => println!("not matched"),
        }
    };
}

fn main() {
    test_match!(x if x > 10, 10); // not matched
    test_match!(n if n > 5, 10); // hello value is 10
    test_match!(Some(x), Some(5)); // hello value is Some(5)
}
