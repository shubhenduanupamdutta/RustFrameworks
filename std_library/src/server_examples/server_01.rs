use std::thread;
use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn read_file(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Ok(content) => format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}",
            content
        ),
        Err(_) => "HTTP/1.1 500 Internal Server Error\r\n\r\n500 Internal Server Error".to_string(),
    }
}

// Function to determine the response based on the request
fn get_response(request: &str) -> Result<String, &'static str> {
    // Check if the request contains "/"
    if request.contains("GET /") {
        // If yes, read the contents of "index.html" and format an HTTP Request
        Ok(read_file("index.html"))
    }
    // Check if the request contains "/second"
    else if request.contains("GET /second") {
        // If yes, read the contents of "second.html" and format an HTTP Request
        Ok(read_file("second.html"))
    }
    // If no matching route is found
    else {
        Err("Route not found")
    }
}

fn handle_client(mut stream: TcpStream) {
    // Buffer to store incoming data
    let mut buffer = [0; 1024];

    // Read the data from the client into the buffer
    if let Err(e) = stream.read(&mut buffer) {
        eprintln!("Failed to read from connection: {}", e);
        return;
    }

    // Convert the buffer to a UTF-8 string
    let request = String::from_utf8_lossy(&buffer[..]);

    // Generate a response based on the request
    let response = match get_response(&request) {
        Ok(content) => content,
        Err(_) => "HTTP/1.1 500 Internal Server Error\r\n\r\n500 Internal Server Error".to_string(),
    };

    // Write the response to the client
    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to write to connection: {}", e);
        return;
    };

    // Flush the output stream
    if let Err(e) = stream.flush() {
        eprintln!("Failed to flush connection: {}", e);
    };
}

// Main function where the server binds to an address and listens for incoming connections
pub fn main() {
    // make a new tcp_listener which is bound to a ip address and port
    // Expect makes sure that if the port or ip is not available, the program will panic
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to create listener");

    println!("Server listening on port 8080...");

    // Accept incoming connection and spawn a new thread to handle each connection
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread to handle each client connection
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Failed to establish a connection: {}", e);
            }
        }
    }
}
