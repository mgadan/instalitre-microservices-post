extern crate openssl;
#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate diesel;

pub mod schema;
pub mod post;
pub mod lib;
pub mod errors;

use lib::establish_connection;
use actix_web::{middleware, App, HttpServer};

fn main() {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();
    
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .data(establish_connection())
            .configure(post::router::config)
    })  
        .bind("0.0.0.0:8000")
        .expect("Can not bind to port 8000")
        .run()
        .unwrap();
}
