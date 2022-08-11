#[macro_use]
extern crate diesel;

mod api;
mod db;
mod middleware;

use crate::api::prelude::*;
use crate::middleware::token_verifier::TokenVerifier;
use actix_web::{App, HttpServer, Scope};
use actix_web_lab::web::spa;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    builder
        .set_private_key_file("localhost+1-key.pem", SslFiletype::PEM)
        .unwrap();

    builder
        .set_certificate_chain_file("localhost+1.pem")
        .unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(
                Scope::new("api/v1")
                    .wrap(TokenVerifier)
                    .service(get_lists)
                    .service(get_items)
                    .service(add_list_item)
                    .service(remove_list_item),
            )
            .service(Scope::new("auth").service(token))
            .service(
                spa()
                    .index_file("./dist/index.html")
                    .static_resources_mount("/")
                    .static_resources_location("./dist")
                    .finish(),
            )
    })
    .bind_openssl("0.0.0.0:80", builder)?
    .run()
    .await
}
