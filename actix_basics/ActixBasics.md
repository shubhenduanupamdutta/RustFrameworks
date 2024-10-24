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

```rust
//! Secure actix web server to handle HTTP Requests over HTTPS using OpenSSL for TLS encryption.
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::io::Result;

async fn index(_req: HttpRequest) -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> Result<()> {
    // Load TLS Keys
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("caKey.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("host.cert").unwrap();

    // Start HTTP Server
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind_openssl("127.0.0.1:8080", builder)?
        .run()
        .await
}
```
- Private Key and Certificate for Localhost can be generated using the following commands:
```bash
openssl genrsa 2048 > host.key
chmod 400 host.key
openssl req -new -x509 -nodes -sha256 -days 365 -key host.key -out host.cert
```
- `SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap()` creates a new instance of SslAcceptor with the Mozilla intermediate security profile.
- `builder.set_private_key_file("caKey.pem", SslFiletype::PEM).unwrap()` sets the private key file for the server.
- `builder.set_certificate_chain_file("host.cert").unwrap()` sets the certificate chain file for the server.
- `HttpServer::new()` creates a new instance of the HttpServer.

---
## Actix Features
---
### _Actix Web has many powerful features, most powerful among them being:_

- **Keep-Alive**
- **Graceful Shutdown**

_These features take care of important areas of server administration, and make our lives as developers easier._

=========================================================
#### Keep-Alive
_Keep-Alive is a feature that allows the same TCP connection to be used for multiple requests/responses, instead of opening a new connection for each request. This can significantly reduce the overhead of establishing new connections and improve the performance of the server._
```rust
use actix_web::{web, App, HttpServer, HttpResponse};

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, Actix with Keep Alive!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(index))
    })
    .keep_alive(75) // Set keep alive timeouts in seconds
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```
- `.keep_alive(75)` sets the keep-alive timeout to 75 seconds. That means the server will keep the connection alive for 75 seconds after the last request/response.

=========================================================
#### Graceful Shutdown
_Graceful Shutdown is a feature that allows the server to finish processing existing requests before shutting down. This ensures that no requests are interrupted or lost during the shutdown process. This ensures that data loss and abrupt disconnection never happens._
```rust
use actix_web::{web, App, HttpServer, HttpResponse};
use std::time::Duration;

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, Actix with Graceful Shutdown!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    .unwrap_or_else(|err| { // We handle graceful shutdown here
        eprintln!("Server Error: {}", err);
        std::process::exit(1);
    })
}
```
_`.run().await.unwrap_or_else(|err| { ... })` handles the graceful shutdown of the server. If an error occurs during the shutdown process, it will print the error message and exit the process. This enhances robustness and reliability of the server._