#[macro_use]
extern crate rocket; // Imports rocket macros in current module

use rocket::config::Config;
use rocket::response::content;

#[get("/")]
fn index() -> content::RawHtml<&'static str> {
    content::RawHtml("<h1>Hello, world, from Simple Rocket Server!</h1>")
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[rocket::main]
async fn main() {
    let config = Config {
        port: 8080,
        ..Config::default()
    };

    rocket::custom(config)
        .mount("/", routes![index, hello])
        .launch()
        .await
        .expect("Failed to launch Rocket server");
}
