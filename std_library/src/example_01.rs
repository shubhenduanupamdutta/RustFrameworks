use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

pub fn main() -> Result<(), Box<dyn Error>> {
    // Open the file
    let file = File::open("example.txt")?;

    // Create a buffered reader to read lines efficiently
    let reader = io::BufReader::new(file);

    // Iterate over lines
    for line in reader.lines() {
        // Unwrap to line or handle any error
        match line {
            Ok(text) => println!("{text}"),
            Err(e) => println!("Error reading line: {e}"),
        }
    }

    Ok(())
}