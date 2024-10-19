//! Vectors
//! Vectors are dynamic arrays, that can grow or shrink in size. They are implemented using generics.

pub fn main() {
    // Creating an empty vector of integers
    let mut numbers: Vec<i32> = Vec::new();

    // Adding elements to a vector
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);

    // Accessing Elements
    println!("Vector: {:?}", numbers);

    // Iterating over elements
    for num in numbers.iter() {
        println!("Number: {}", num);
    }
}