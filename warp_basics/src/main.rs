//! Warp Server
use warp::Filter;

#[tokio::main]
async fn main() {
    // Define a simple filter that responds to a specific path
    let hello = warp::path!("hello" / "world").and_then(|| {
        // Introduces and asynchronous operation in the handler
        async { Ok::<_, warp::Rejection>(warp::reply::html("Hello, Warp!")) }
    });

    // Start the warp server with our filter
    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
