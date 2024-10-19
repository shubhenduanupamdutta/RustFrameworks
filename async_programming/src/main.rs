//! Example of async programming in Rust
//! A program that concurrently fetches multiple web pages using async
//! 
use tokio::time::{sleep, Duration};
use tokio;
use reqwest;

#[tokio::main]
async fn main() {
    // A list of URLs to fetch concurrently
    let urls = vec![
        "https://www.example.com",
        "https://www.rust-lang.org",
        "https://www.github.com",
        "https://www.youtube.com",
    ];

    // Create a vector to store the spawned task handles
    let mut handles = Vec::new();

    // Spawn asynchronous tasks to fetch each URL concurrently
    for url in urls {
        let handle = tokio::spawn(fetch_url(url));
        handles.push(handle);
    }

    // Wait for all tasks to complete
    tokio::join!(
        async {
            // Concurrently fetch URLs
            futures::future::join_all(handles).await;
        },
        async {
            // Perform other asynchronous tasks concurrently
            println!("Doing other asynchronous tasks concurrently...");
            sleep(Duration::from_secs(5)).await;
            println!("Other tasks completed!");
        }
    );
}

async fn fetch_url(url: &str) {
    // Asynchronously fetch a URL using reqwest
    let response = reqwest::get(url).await;

    // Print the result or an error message
    match response {
        Ok(response) => println!("Successfully fetched {}: {:?}", url, response.status()),
        Err(error) => eprintln!("Failed to fetch {}: {:?}", url, error),
    }
}