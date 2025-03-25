// use clap::{Parser, Subcommand};

// /// MyCLI: A simple command-line tool
// #[derive(Parser)]
// #[command(name = "mycli")]
// #[command(version = "1.0")]
// #[command(about = "A simple CLI calculator", long_about = None)]
// struct Cli {
//     #[command(subcommand)]
//     command: Commands,
// }

// #[derive(Subcommand)]
// enum Commands {
//     /// Adds two numbers
//     Add { a: i32, b: i32 },

//     /// Subtracts two numbers
//     Subtract { a: i32, b: i32 },

//     /// Multiplies two numbers
//     Multiply { a: i32, b: i32 },
// }

// fn main() {
//     let cli = Cli::parse();

//     match cli.command {
//         Commands::Add { a, b } => println!("Result: {}", a + b),
//         Commands::Subtract { a, b } => println!("Result: {}", a - b),
//         Commands::Multiply { a, b } => println!("Result: {}", a * b),
//     }
// }
use clap::{Command, Parser, Subcommand};
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { a: i32, b: i32 },
    Subtract { a: i32, b: i32 },
}
fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Add { a, b } => print!("addition of two numbes is{}", a + b),
        Commands::Subtract { a, b } => print!("subtract of two numbes is{}", a - b),
    }
}
