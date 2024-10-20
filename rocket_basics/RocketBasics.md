# Rocket - Web Framework

---

### _**Rocket** is a web framework for rust that makes it simple to write fast, type-safe and secure web applications with incredible usability, productivity and performance._

---

## What is Rocket?

_Rocket is a web framework that prioritizes type-safe routes and clean syntax. Because of its extensible design and support for middleware and plugins, it enables us to create scalable and responsive web applications._

_Rocket makes your web apps, performant and dependable by leveraging and building upon Rust asynchronous capabilities and safety features._

---

## Importing Rocket

```toml
[dependencies]
rocket = "0.5.1"
```

or

```bash
cargo add rocket
```

---

## Key Features of Rocket

- ### **Extensibility** - _Because of the framework's high degree of extensibility, middleware and plugins can be used by developers to increase functionality._
- ### **Asynchronous Support** - _Rocket facilitates async programming by leveraging the asynchronous syntax of Rust to create scalable and dependable web apps._
- ### **Type-safe routes** - _Rocket's type-safe routes improve code dependability and lower the possibility of runtime errors._
- ### **Macro-based Syntax** - _Rocket's macro-based syntax makes it simple to write clean and readable code._

---

## Why's Rocket effective?

- ### _Defining routes and handling requests are intuitive and straightforward._
- ### _Its type-safe routes and effortless integration with Rust makes project development straightforward. It streamlines development of project, allowing us to focus on application logic rather than intricate web details._
- ### _Rocket's emphasizes safety and performance by leveraging Rust's ownership and borrowing system to enforce memory safety and prevent common pitfalls such as null pointer, data races etc._
- ### _Rocket's compile time checks and optimizations help produce efficient and reliable web applications._

---

## Rocket Important Concepts

---

### Lifecycle

========================================================

#### _**Lifecycle** encompasses the stages the HTTP request goes through from the coming of the request to the going of the response for that request._

#### Brief breakdown of all the stages in the lifecycle:

- **Request Parsing** - _Here the request is parsed to extract the HTTP method, content type, headers and the data pact with the request._
- **Routing** - _The request is matched to a route and the corresponding handler is called._
- **Handler Execution** - _Once the request is routed to the correct handler, the handler is executed._
- **Response Generation** - _The handler generates a response and sends it back to the client, where it can be displayed or processed._
- **Response Serialization** - _The response is serialized into a format that can be sent over the network, such as JSON or HTML._
- **Response Sending** - _The response is sent back to the client, completing the lifecycle._

========================================================

### Routing

========================================================

#### _**Routing** - \_In routing, we map the HTTP request to the request handling function for handling the request. There are `5` routing strategies in Rocket._

#### _**Routing Strategies**_

- **Basic Routing**
- **Dynamic Routing**
- **Routing with Query Parameters**
- **Multiple Routes**
- **Routes with Guards**

#### Basic Routing Example

```rust
#[get("/")]
fn index() -> &'static str {
    "Welcome to the Rocket Web Framework's homepage."
}
```

_Above route responds to get requests to the route URL and returns a static string._

#### Dynamic Routing Example

```rust
#[get("/hello/<name>")]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

_Above route responds to get requests to the route URL and returns a dynamic string. Because it uses a dynamic parameter, `name` it can greet any name._

_`name` is dynamic path parameter (i.e. it can be any string). Its value will be extracted from the URL and passed as a parameter to the greet function._

#### Query Parameters Example

```rust
#[get("/search?<query>)"]
fn search(query: String) -> String {
    format!("Searching for {}", query)
}
```

_With query parameters, this responds to get requests to `/search` with query parameters. The query parameter is extracted from the URL and passed as a parameter to the search function._

#### Multiple Routes Example

```rust
#[get("/")]
fn index() -> &'static str {
    "Welcome to the Rocket Web Framework's homepage."
}

#[get("/about")]
fn about() -> &'static str {
    "About Rocket Web Framework"
}
```

_Above is example of multiple routing. This example has two routes, one for the root url `/`, and another for `/about`. Each route corresponds to a separate handler function, that returns a different response._

#### Routes with Guards Example

```rust
#[get("/admin")]
fn admin_panel() -> &'static str {
    "Admin Panel"
}

#[get("/admin", rank = 2)]
fn unauthorized_admin() -> &'static str {
    "Unauthorized Access to the admin panel"
}

#[catch(404)]
fn not_found() -> &'static str {
    "Page not found"
}
```

_Above code is example for routes with guards. In this example, there are two routes for the same URL `/admin`. The first route is for authorized access to admin panel, and the second route is fallback route for unauthorized access._

_Additionally, there's also a catch all route for 404 errors, that will be called when no other route matches._

========================================================
### Mounting
========================================================

#### _This is all about organizing and structuring the routes in your web application. Mounting allows you to group routes together and apply common attributes to them. There are three ways to do this in Rocket._

#### Ways of mounting routes are:
- **Mounting routes at root**
- **Mounting routes at a specific path**
- **Nested Mounting**

#### Mounting at root

```rust

#[get("/")]
fn index() -> &'static str {
    "Welcome to the Rocket Web Framework's homepage."
}

#[get("/about")]
fn about() -> &'static str {
    "About Rocket Web Framework"
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![index, about])
        .launch()
        .await
        .expect("Failed to launch Rocket Web Framework");
}
```
_In above example, both, index and the about routes are mounted at the root path `/`. This means they will respond to requests made to the root URL and `/` and `/about` respectively._

#### Mounting at a specific path

```rust
#[get("/hello")]
fn hello() -> &'static str {
    "Hello, World!"
}

#[get("/api")]
fn api() -> &'static str {
    "API Endpoint"
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/app", routes![hello, api])
        .launch()
        .await
        .expect("Failed to launch Rocket Web Framework");
}
```
_In above example, hello and api routes are mounted at the `/app` path. This means they will respond to requests made to the `/app/hello` and `/app/api` respectively._

#### Nested Mounting

```rust
#[get("/")]
fn index() -> &'static str {
    "Welcome to the Rocket Web Framework's homepage."
}

#[get("/about")]
fn about() -> &'static str {
    "About Rocket Web Framework"
}

#[get("/contact")]
fn contact() -> &'static str {
    "Contact Us"
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![index])
        .mount("/info", routes![about, contact])
        .launch()
        .await
        .expect("Failed to launch Rocket Web Framework");
}
```
_In above example, the index route is mounted at the root path `/`, and the about and contact routes are mounted at the `/info` path. This means the index route will respond to requests made to the root URL, and the about and contact routes will respond to requests made to the `/info/about` and `/info/contact` respectively._

========================================================
### Edge
========================================================
#### _**Edge** refers to experimental features and syntax extension that enables developers to extend the functionality of Rocket using cutting edge functionality that may not yet be stable or widely adopted. Enabling edge features allows developers to experiment with new language and library features as they evolve towards stability. There are three ways this can be done._

#### Ways of using edge features are:
- **Using Experimental syntax**
- **Using edge features for Rocket configuration**
- **Using experimental features in Route definition**

#### Using Experimental syntax

```rust
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket

#[get("/")]
fn index() -> &'static str {
    "Welcome to the Rocket Web Framework's homepage."
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![index])
        .launch()
        .await
        .expect("Failed to launch Rocket Web Framework");
}
```
_In above example, the `#![feature(proc_macro_hygiene, decl_macro)]` attribute is used to enable experimental syntax features. This allows the use of experimental syntax features in the code. They require explicit opt-in using `#![feature]` attribute, because they are experimental and not yet stable._

#### Using edge features for Rocket configuration

```rust
#![feature(decl_macro)]

#[macro_use] extern crate rocket

use rocket::config::{Config, Environment};

#[get("/")]
fn index() -> &'static str {
    "Welcome to the Rocket Web Framework's homepage."
}

#[rocket::main]
async fn main() {
    let config = Config::build(Environment::Development)
        .address("127.0.0.1")
        .port(8000)
        .finalize()
        .unwrap();

    rocket::custom(config)
        .mount("/", routes![index])
        .launch()
        .await
        .expect("Failed to launch Rocket Web Framework");
}
```

_In above example we have used `Config::build` method to create a custom configuration for Rocket. This allows us to configure the Rocket instance with custom settings such as the address and port to listen on. Again they will require opt-in into edge features._

#### Using experimental features in Route definition

```rust
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket

#[get("/")]
fn index() -> &'static str {
    "Welcome to the Rocket Web Framework's homepage."
}

#[get("/hello/<name>")]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[rocket::main]
async fn main() {
    rocket::ignite()
        .mount("/", routes![index, greet])
        .launch()
        .await
        .expect("Failed to launch Rocket Web Framework");
}
```
_In above example, we have used experimental features in route definition. The `#![feature(proc_macro_hygiene, decl_macro)]` attribute is used to enable experimental syntax features in the route definition. This allows the use of experimental syntax features in the route definition. They require explicit opt-in using `#![feature]` attribute, because they are experimental and not yet stable._