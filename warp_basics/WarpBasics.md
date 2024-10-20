# Warp - A Rust Web Framework - Basics

---
## What is warp?
- ### Designed for speed and usability in Rust.
- ### Excellent use of the async/await syntax and features.
_Warp at its core, is a web framework designed for speed an usability in Rust. What makes it truly remarkable is its excellent use of async await syntax, offering seamless asynchronous handling._

---
## Importing warp
```toml
[dependencies]
warp = "0.3"
```
```bash
cargo add warp
```

---
## Key Features of Warp
- **Asynchronous Power** - _Warp is built on top of the async/await syntax in Rust, allowing you to write asynchronous code that reads like synchronous code._
- **Filters** - _Filters in Warp are powerful building blocks for defining routes. They allow you to match, transform and combine requests easily._
- **Functional and Composable** - _Warp encourages a functional programming style, allowing you to compose complex APIs using small, reusable components._
- **Expressive Syntax** - _Warp makes it easy to define routes and handle requests with a clean and expressive syntax._
- **Comprehensive Documentation** - _Warp comes with excellent documentation that makes it easy to get started and build complex applications._

---
## `Filters`
_Filters in Warp act as potent building blocks by defining how you define routes. A filter can optionally extract some data from a request, combine it with others, mutate it and return back some value as a reply._

_Power of filters comes from being able to isolate small subsets of your application logic and then chain and reuse them in various parts of your app._

**Filters are used to:**
- **Match Requests** - _Identify and respond to specific types of requests._
- **Transform Data** - _Modify the request or response data seamlessly._
- **Compose Routes** - _Combine multiple filters to create complex routes. You can assemble intricate routes by combining and reusing filters._
---