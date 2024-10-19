//! HTTP Post Server with Tide
use serde::{Deserialize, Serialize};
use tide::{prelude::*, Response, StatusCode};

#[derive(Deserialize, Serialize)]
struct UserData {
    name: String,
    age: u32,
}

async fn handle_post(mut request: tide::Request<()>) -> tide::Result {
    // Extract JSON data from the request body
    let user_data: UserData = request.body_json().await?;

    // Validate the incoming JSON data
    if user_data.name.is_empty() {
        let mut response = Response::new(StatusCode::BadRequest);
        response.set_body("Name cannot be empty.");
        return Ok(response);
    }

    if user_data.age <= 0 {
        let mut response = Response::new(StatusCode::BadRequest);
        response.set_body("Age cannot be negative.");
        return Ok(response);
    }

    // Respond with a confirmation message
    Ok(json!({
        "message": "Data Received Successfully.",
        "user": user_data
    })
    .into())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    // Define a route to handle POST requests
    app.at("/post").post(handle_post);

    // Start the server at http://
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
