use std::net::TcpListener;

pub fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to create listener");

    // Accept incoming connection
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Accepted connection from: {}", stream.peer_addr().unwrap());
            }
            Err(e) => {
                eprintln!("Failed to establish a connection: {}", e);
            }
        }
    }
}
