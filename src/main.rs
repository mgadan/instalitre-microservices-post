#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};
mod post;

fn main() {
    let _guard = sentry::init(None);

    HttpServer::new(|| {
        App::new()
            .configure(post::router::config)
    })
        .bind("0.0.0.0:8000")
        .expect("Can not bind to port 8000")
        .run()
        .unwrap();
}
