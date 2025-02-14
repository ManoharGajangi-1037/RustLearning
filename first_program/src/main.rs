struct Employee{
    name:String,
    id:u64,
}
fn main() {
    let employee_1=Employee{
        id:7,
        name:"manu".to_string()
    };
    println!("First Employee Name :{}",employee_1.name);
}
