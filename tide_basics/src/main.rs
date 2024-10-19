//! Basic Tide Server

async fn hello(req: tide::Request<()>) -> tide::Result {
    println!("Received request: {:#?}", req);
    Ok("Hello, Tide!".into())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("/hello").get(hello);

    app.listen("127.0.0.1:8000").await?;
    Ok(())
}
