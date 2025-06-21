fn get_user_name()->Result<&'static str,()>{
   Ok("hello")
}

fn first_name()->Result<String,()>{
    Err(())
}
fn last_name()->Result<String,()>{
   Ok("Manohar".to_string())
}

fn full_name()->Result<String,()>{
    let first_name=first_name()?;
    let last_name=last_name()?;
    Ok(format!("{} {}",first_name,last_name))
}
fn main() {
    // let user_name=get_user_name();
    // match user_name{
    //     Ok(value)=>println!("{}",value),
    //     Err(_)=>println!("Error occured")
    // }

    //  let user_name=get_user_name().expect("error occured");
    let full_name=full_name();
    match full_name{
        Ok(name)=>println!("{}",name),
        Err(_)=>println!("Error Occured")
    }
}
