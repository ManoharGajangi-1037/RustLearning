use std::{fs::File, process};
use std::io::{stdin, Read};
fn main() {
    let mut name = String::new();
    let  pathname = match stdin().read_line(&mut name){
        Ok(_)=> name.trim(),
        Err(error)=>{
            eprintln!("error occured {error:?} ");
            process::exit(1);
        }
    };

    let mut file = match File::open(pathname){
        Ok(file)=>file,
        Err(error)=>{
            eprintln!("error occured {error} ");
            process::exit(1);
        }
    };

    let mut file_contents = String::new();

    let result = file.read_to_string(&mut file_contents);

    if let Err(error) = result{
         eprintln!("Error occured while reading file contents {error}");
         process::exit(1);
    };
    println!("Contents of the file ::{file_contents}")
}
