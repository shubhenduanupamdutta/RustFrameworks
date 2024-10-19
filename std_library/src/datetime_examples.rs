//! Examples of how to use the date and time in Rust.

use std::time::SystemTime;

pub fn main() {
    // Get the current system time
    let now = SystemTime::now();

    // Some work can be done here, which we can measure time taken
    if let Ok(elapsed) = now.elapsed() {
        println!("Time taken: {:?}", elapsed);
    } else {
        println!("Error occurred while measuring time");
    }
}