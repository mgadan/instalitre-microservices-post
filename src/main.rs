#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate diesel;
pub mod schema;
use actix_web::{App, HttpServer};
pub mod post;

fn main() {
    HttpServer::new(|| {
        App::new()
            .configure(post::router::config)
    })
        .bind("0.0.0.0:8000")
        .expect("Can not bind to port 8000")
        .run()
        .unwrap();
}
