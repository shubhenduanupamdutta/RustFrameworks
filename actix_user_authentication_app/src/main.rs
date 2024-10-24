//! User Authentication App using Actix Web

use actix_web::{
    post,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// A User Struct
#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    password: String,
    email: String,
}

/// Shared state for managing users (In production, use a database)
type Users = Arc<Mutex<Vec<User>>>;

// Register a new user
#[post("/register")]
async fn register_user(user: web::Json<User>, users: web::Data<Users>) -> impl Responder {
    let mut users = users.lock().unwrap();
    if users.iter().any(|u| u.username == user.username) {
        return HttpResponse::Conflict().body("Username already exists");
    }
    users.push(user.into_inner());
    HttpResponse::Ok().body("User registered successfully")
}

/// Login a user
async fn login_user(user: web::Json<User>, users: web::Data<Users>) -> impl Responder {
    let users = users.lock().unwrap();
    if let Some(found_user) = users
        .iter()
        .find(|u| u.username == user.username && u.password == user.password)
    {
        HttpResponse::Ok().json(found_user)
    } else {
        HttpResponse::Unauthorized().body("Invalid username or password")
    }
}

/// Update user account
async fn update_account(user: web::Json<User>, users: web::Data<Users>) -> impl Responder {
    let mut users = users.lock().unwrap();
    if let Some(found_user) = users.iter_mut().find(|u| u.username == user.username) {
        *found_user = user.into_inner();
        HttpResponse::Ok().body("User account updated successfully")
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

/// Main Function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create a shared state for managing users
    let users: Users = Arc::new(Mutex::new(Vec::new()));

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(users.clone()))
            .service(register_user)
            .route("/login", web::post().to(login_user))
            .route("/account/update", web::post().to(update_account))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
