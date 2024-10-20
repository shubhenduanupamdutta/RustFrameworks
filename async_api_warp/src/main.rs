//! Challenge: Create an API that can handle async requests and returns
//! employees name when provided their ID.
//!
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex as AsyncMutex;
use warp::{Filter, Reply};

// Define a simple employee struct
#[derive(Debug, Serialize, Deserialize)]
struct Employee {
    id: u32,
    name: String,
}

// Define a hashmap to store employee data
type EmployeeMap = Arc<AsyncMutex<HashMap<u32, String>>>;

// Function to fetch employee's name asynchronously
async fn fetch_employees_name(id: u32, employees: EmployeeMap) -> Option<String> {
    let guard = employees.lock().await;
    guard.get(&id).cloned()
}

// Api handler to handle requests for employee's name
async fn get_employee_name(id: u32, employees: EmployeeMap) -> Result<impl Reply, warp::Rejection> {
    match fetch_employees_name(id, employees).await {
        Some(name) => Ok(warp::reply::json(&Employee { id, name })),
        None => Ok(warp::reply::json(&format!(
            "Employee with id {} not found",
            id
        ))),
    }
}

#[tokio::main]
async fn main() {
    let employees: EmployeeMap = Arc::new(AsyncMutex::new(HashMap::new()));

    // Insert some dummy data
    {
        let mut guard = employees.lock().await;
        guard.insert(1, "Alice".to_string());
        guard.insert(2, "Bob".to_string());
        guard.insert(3, "Charlie".to_string());
    }

    // Define a filter for handling requests to fetch employee name
    let get_employee_name_route = warp::path!("employees" / u32)
        .and(warp::get())
        .and(warp::any().map(move || employees.clone()))
        .and_then(get_employee_name);

    // Start the warp server with our route
    warp::serve(get_employee_name_route)
        .run(([127, 0, 0, 1], 8080))
        .await;
}
