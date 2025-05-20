fn main() {
    let mut name = String::from("Alice");

    // Immutable borrow
    print_name(&name); // Allowed
    // Mutable borrow
    change_name(&mut name); // Also allowed, but not at the same time as an immutable borrow

    println!("Updated name: {}", name); // Ownership remains with 'main'
}

fn print_name(n: &String) {
    println!("Name is: {}", n);
}

fn change_name(n: &mut String) {
    n.push_str(" Smith");
}

