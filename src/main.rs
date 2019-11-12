extern crate openssl;
#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};
mod post;

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
