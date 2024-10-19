//! Queues - VecDeque
//! Queues are important for managing tasks like request-response queues in a web server, task scheduling, etc.
//! The VecDeque is a double-ended queue implemented using a growable ring buffer.
//! 

pub fn main() {
    // Create a double-ended queue of integers
    let mut deque = std::collections::VecDeque::new();

    // Add elements to the front
    deque.push_back(1);
    deque.push_back(2);

    println!("Deque: {:?}", deque);
    
    // Dequeuing from the front
    if let Some(front) = deque.front() {
        println!("Dequeuing from the front: {}", front);
    }
}