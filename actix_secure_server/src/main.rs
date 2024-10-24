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
