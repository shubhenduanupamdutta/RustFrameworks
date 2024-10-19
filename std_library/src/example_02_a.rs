//! Example for Arrays
//! Arrays are to be used when we have fixed number of elements

pub fn main() {
    // Creating an array of integers
    let numbers: [i32; 3] = [1, 2, 3];

    // Accessing Elements
    println!("Array: {:?}", numbers);

    // Iterating over elements,
    for num in numbers.iter() {
        println!("Number: {}", num);
    }
}