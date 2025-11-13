

//Here if we use option of i32 ,we can use value again even after we matching ,
//when we use option of string we can't as it is related to heap ,no copy trait will be implemented
//Same thing happens with Result Enum 
fn some_function(is_true:bool)->Option<i32>{
    if is_true {
        Some(32)
    }else{
        None
    }
}
fn main() {
    let value = some_function(true);

    let content = match value {
        Some(value)=>value,
        None=> 0
    };
    println!("{:?}",value);
}
