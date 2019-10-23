#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate diesel;

pub mod schema;
pub mod errors;
pub mod post;
pub mod lib;

use lib::establish_connection;
use actix_web::{App, HttpServer};

fn main() {
    HttpServer::new(|| {
        App::new()
            .data(establish_connection())
            .configure(post::router::config)
    })  
        .bind("0.0.0.0:8000")
        .expect("Can not bind to port 8000")
        .run()
        .unwrap();
}
