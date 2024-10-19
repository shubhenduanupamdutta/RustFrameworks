// //! Basic Tide Server

// async fn hello(req: tide::Request<()>) -> tide::Result {
//     println!("Received request: {:#?}", req);
//     Ok("Hello, Tide!".into())
// }

// #[async_std::main]
// async fn main() -> tide::Result<()> {
//     let mut app = tide::new();

//     app.at("/hello").get(hello);

//     app.listen("127.0.0.1:8000").await?;
//     Ok(())
// }

/**
 * Table of Content Routing
 */

// async fn hello(_: tide::Request<()>) -> tide::Result {
//     Ok("Hello, Table of Contents Routing!".into())
// }

// async fn greet(_: tide::Request<()>) -> tide::Result {
//     Ok("Greetings from the sub-route!".into())
// }

// #[async_std::main]
// async fn main() -> tide::Result<()> {
//     let mut app = tide::new();

//     app.at("/hello").get(hello);
//     app.at("/hello/greet").get(greet);

//     app.listen("127.0.0.1:8000").await?;
//     Ok(())
// }

/**
 * Freeform Composition Routing
 */

async fn hello(_: tide::Request<()>) -> tide::Result {
    Ok("Hello, Free-Form composition Routing!".into())
}

async fn greet(_: tide::Request<()>) -> tide::Result {
    Ok("Greetings from the free-form route!".into())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("/hello").get(hello);
    app.at("/greet").get(greet);

    app.listen("127.0.0.1:8000").await?;
    Ok(())
}
