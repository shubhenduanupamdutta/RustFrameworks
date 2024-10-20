//! Rest API Web Server with Warp
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use warp::Filter;

// Define a simple data structure to represent our resources
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Item {
    id: Option<u64>,
    name: String,
}

#[tokio::main]
async fn main() {
    // Initialize the vector to store multiple items
    let items: Arc<Mutex<Vec<Item>>> = Arc::new(Mutex::new(Vec::new()));

    // Define a filter to create a new item
    let create_item = warp::path!("items" / "create")
        .and(warp::post())
        .and(warp::body::json())
        .map({
            let items = items.clone();
            move |new_item: Item| {
                let mut items = items.lock().unwrap();
                let id = items.len() as u64 + 1;
                let item = Item {
                    id: Some(id),
                    name: new_item.name,
                };
                items.push(item.clone());
                warp::reply::json(&item)
            }
        });

    // Define a filter to get all items
    let get_all_items = warp::path!("items" / "all").and(warp::get()).map({
        let items = items.clone();
        move || {
            let items = items.lock().unwrap();
            warp::reply::json(&items.clone())
        }
    });

    // Combine filters into the main API
    let api = create_item.or(get_all_items);

    // Run the server
    warp::serve(api).run(([127, 0, 0, 1], 8080)).await;
}
