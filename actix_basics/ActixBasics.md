# Actix Web - Rust Web Framework - Basics

---
### _Actix Web is a powerful, pragmatic, and extremely fast web framework for Rust. It is a robust Rust framework that uses actors, and renowned for its scalability and great performance._

---
## Adding Actix Web to your project
```toml
[dependencies]
actix = "0.13.5"
actix-web = "4.9.0"
```
or
```bash
cargo add actix-web
```
---
## Actix: Important Features
---
### Asynchronous and non-blocking
_Actix's ability to handle asynchronous programming is one of its best qualities. Because actix-web is built on top of the Actix actor framework, it is able to provide highly concurrent and asynchronous runtime. It allows handling thousand of connection with minimal overhead by utilizing asynchronous I/O and non-blocking operations._

### Actor Based Programming Model
_Actix Web follows, Actor Based Programming Model, where components of the application are organized as actors. Actors communicate through each other through message passing, which enables easy concurrency and parallelism without shared mutable state reducing the risk of data races and synchronization issues._

---

## Basic HTTP Server using Actix Web
---

```rust
//! Actix Web Simple Server
use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    "Hello World from Actix Web!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```
- `impl Responder` indicating that the return type of the function is a type that implements the Responder trait. Which says that this type can be converted to an HTTP response.
- `#[actix_web::main]` is a procedural macro that sets up the Actix system and starts the main function.
- `HttpServer::new()` creates a new instance of the HttpServer.
- `App::new().service(index)` creates a new instance of the App and registers the index service.
- `.bind("127.0.0.1:8080")?` binds the server to the specified IP address and port.
- `.run().await` starts the server and waits for it to finish.

---
## Actix TLS
---
_In this section, we will explore how to add TLS support to an Actix Web server. TLS (Transport Layer Security) is a cryptographic protocol that provides secure communication over a computer network. It is widely used to secure web traffic and protect sensitive data._
