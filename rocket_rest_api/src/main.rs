//! REST API for Rocket

#[macro_use] extern crate rocket;

use rocket::State;
use rocket::serde::json::{Json, Value as JsonValue, json};
use std::sync::Mutex;
use rocket::serde::{Deserialize, Serialize};

/// Define a struct to represent a User
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct User {
    id: u32,
    name: String,
    email: String,
}

/// Create a shared state to store users (Using Mutex to make it thread-safe)
type UserDB = Mutex<Vec<User>>;

/// Add a user to the database
#[post("/users", format = "json", data="<user>")]
async fn add_user(user: Json<User>, users_db: &State<UserDB>) -> JsonValue {
    let mut users = users_db.lock().unwrap();
    users.push(user.into_inner());
    json!({"status": "success", "message": "User added successfully"})
}

/// Get all users from the database
#[get("/users")]
async fn get_users(users_db: &State<UserDB>) -> Json<Vec<User>> {
    let users = users_db.lock().unwrap();
    Json(users.clone())
}

/// Update a user in the database by ID
#[put("/users/<id>", format = "json", data = "<user>")]
async fn update_user(id: u32, user: Json<User>, users_db: &State<UserDB>) -> JsonValue {
    let mut users = users_db.lock().unwrap();
    if let Some(u) = users.iter_mut().find(|u| u.id == id) {
        *u = user.into_inner();
        return json!({"status": "success", "message": "User updated successfully"});
    } 
    json!({"status": "error", "message": "User not found"})
}

/// Delete a user from the database by ID
#[delete("/users/<id>")]
async fn delete_user(id: u32, users_db: &State<UserDB>) -> JsonValue {
    let mut users = users_db.lock().unwrap();
    let initial_len = users.len();
    users.retain(|u| u.id != id);
    if users.len() < initial_len {
        return json!({"status": "success", "message": "User deleted successfully"});
    }
    json!({"status": "error", "message": "User not found"})
}

/// Rocket App Configuration
#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![add_user, get_users, update_user, delete_user])
        .manage(Mutex::new(Vec::<User>::new()))
        .launch()
        .await
        .expect("Failed to start rocket server.");
}