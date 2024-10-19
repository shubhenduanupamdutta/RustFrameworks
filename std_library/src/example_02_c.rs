//! HashMaps
//! HashMaps are key-value stores, that is, they store a value associated with a key. The key is used to look up the value. The key must be unique within the HashMap. The value can be any type. HashMaps are part of the standard library and are defined in the collections module.
//! They provide a way to store data in a way that allows for fast lookup. HashMaps are implemented using generics.
//! In Web development, HashMaps are used to store session data, user data, and other data that needs to be accessed quickly.

use std::collections::HashMap;

pub fn main() {
    let mut user_age: HashMap<&str, u32> = HashMap::new();

    // Inserting elements
    user_age.insert("Alice", 30);
    user_age.insert("Bob", 25);
    user_age.insert("Charlie", 20);

    // Accessing values by key
    if let Some(age) = user_age.get("Bob") {
        println!("Bob's age: {}", age);
    }

    // Iterating over key-value pairs
    for (name, age) in &user_age {
        println!("{name} is {age} years old");
    }

    println!("All users: {:?}", user_age);
}