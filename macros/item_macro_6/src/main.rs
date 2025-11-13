macro_rules! define_items {
    ($sample:item) => {
        $sample
    };
}

//We can have this too by defining items in the main!
// struct Student {
//     name: i32,
// }
macro_rules! print_student {
    ($student:expr) => {
        println!("{} {}", $student.name, $student.id);
    };
}

macro_rules! default_struct {
    ($($car:item),*) => {
        $($car)*
    };
}

default_struct!(
    struct Car {},
    impl Car{
        fn drive_test() {
            println!("Hello World");
        }
    }
);
fn main() {
    define_items!(
        struct Student {
            name: String,
            id: i32,
        }
    );
    let student = Student {
        name: "Manohar".to_string(),
        id: 32,
    };
    print_student!(student);
    Car::drive_test();
}
