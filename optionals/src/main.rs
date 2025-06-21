fn main() {
    /*
      let age=Some(20);
    if let Some(age)=age{
       println!("{}",age);
    }
    */

    let age = Some(20);
    let ans = age.unwrap();
    //It will give answer if existed ,If it has None then it will panic
    //Inorder to not panic we can use unwrap_or("mention some value here");
    /*
     let age2:Option<i32>=None;
     let ans2=age2.expect("hello");
     print!("{}",ans2)

     It will panic and print the message,if None is provided,else it will print the data
    */
}
