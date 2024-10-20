# Basics of Tide Framework

---

## Key Features of Tide Framework

<h3>
    <ul>
        <li> Asynchronous - <em>Tide is built from the ground up to take full advantage of Rust's asynchronous capabilities. Which enables it to handle large number of concurrent connection.</em>
        </li>
        <br>
        <li> Minimalistic - <em>Tide follows the principal of minimalism, providing a clean and intuitive API that makes it easy to get started while still offering flexibility for complex application.</em></li>
        <br>
        <li> Type-safe Routing - <em>With tide you can define routes in a type safe manner, reducing the likelihood of runtime errors and enhancing code maintainability.</em></li>
        <br>
        <li> Middleware Support - <em>Tide supports middleware allowing us to plugin additional functionality in request response lifecycle. </em></li>
        <br>
        <li> No Global State - <em> Tide adopts a stateless model eliminating global mutable state, and encouraging a more predictable and maintainable codebase.</em></li>
    </ul>
</h4>

---
## Tide Routing Strategies
---
### Endpoint-centric Routing
_This strategy revolves around defining handlers for specific endpoints. Each route is associated with a unique handler allowing you for a straightforward and clear structure in your code._
```rust
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
```
---
### Table of contents routing
_This approach involves grouping routes based on a common prefix. It provides a hierarchial structure making it excellent for organizing APIs with multiple endpoints._

```rust
use tide::prelude::*;

async fn hello(_: tide::Request<()>) -> tide::Result {
    Ok("Hello, Table of Contents Routing!".into())
}

async fn greet(_: tide::Request<()>) -> tide::Result {
    Ok("Greetings from the sub-route!".into())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("/hello").get(hello);
    app.at("/hello/greet").get(greet);

    app.listen("127.0.0.1:8000").await?;
    Ok(())
}
```
_In above example we have a base route at `/hello` and a sub-route at `/hello/greet`. The **greet** handler is nested under `/hello` prefix. This grouping enhances the organization of our APIs._

---
### Routing with free-form composition
---
_This strategy provides maximum flexibility allowing us to compose routes in a free-form manner. This is particularly useful for complex applications where routes are not easily categorized and need intricate routes._

```rust
/**
 * Freeform Composition Routing
 */

async fn hello(_: tide::Request<()>) -> tide::Result {
    Ok("Hello, Free-Form composition Routing!".into())
}

async fn greet(_: tide::Request<()>) -> tide::Result {
    Ok("Greetings from the free-form route!".into())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("/hello").get(hello);
    app.at("/greet").get(greet);

    app.listen("127.0.0.1:8000").await?;
    Ok(())
}
```
_With free-form composition you can define routes independently, allowing you for more dynamic composition of routes._
_In the above example we have two routes `/hello` and `/greet` which are not nested under any common prefix. And each route have their own handler functions._

---
## Use Cases for Tide
---
### Characteristics of Tide
- Lightweight
- Asynchronous
- Simple
- Flexible
- Minimalistic

_Tide is a lightweight asynchronous web-framework designed for simplicity and flexibility. Its a great fit when you need a minimalistic framework with excellent support for synchronous operations._

### Use Cases
- Microservices
- Prototyping and Rapid Development
- Learning Async Rust 

---
## Limitations of Tide
---
- #### Lack of batteries-included features
    _Tide is a minimalistic framework and does not provide batteries-included features like ORM, Authentication, etc. You will need to integrate third-party libraries to add these features._
- #### Community Maturity
    _Tide is a relatively new framework and has a smaller community compared to other Rust web frameworks. This may result in slower adoption and fewer resources available._
- #### Enterprise-grade Features
    _Tide may not be the best choice for enterprise-grade applications that require extensive features, like built-in authentication, complex middleware support and extensive documentation and support. You may need to evaluate other frameworks like Actix-web or Rocket for such use-cases._

---
## Conclusion
### _Tide is a fantastic choice for specific use cases where its strengths align with your project requirements. Understanding its limitations help you make informed decisions when selecting a framework._ 