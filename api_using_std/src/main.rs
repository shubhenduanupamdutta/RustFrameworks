//! Building an API Server using Rust's standard library
//!

use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    println!("Server listening on 127.0.0.1:8000...");

    for stream in listener.incoming().take(5) {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    // handle client connection
    let mut buffer = [0; 1024];

    if let Err(e) = stream.read(&mut buffer) {
        eprintln!("Failed to read from connection: {}", e);
        return;
    }

    let request = String::from_utf8_lossy(&buffer[..]);

    if request.contains("GET /api") {
        let response = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"message\": \"Hello, Rust API!\"}";

        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Failed to write to connection: {}", e);
            return;
        } 
    } else {
        let response = "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/plain\r\n\r\nNot Found";

        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Failed to write to connection: {}", e);
            return;
        }

    }

    // Flush the stream to ensure all data is sent
    if let Err(e) = stream.flush() {
        eprintln!("Failed to flush stream: {}", e);
        return;
    }

}
