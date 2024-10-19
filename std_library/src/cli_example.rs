//! Accessing command line arguments

use std::env;

pub fn main() {
    // Access Command Line Arguments
    let args: Vec<String> = env::args().collect();

    // Print the command line arguments
    println!("Program: {}", args[0]);
    println!("Arguments: {:?}", &args[1..]);
}