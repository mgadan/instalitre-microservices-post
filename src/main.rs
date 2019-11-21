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
use crate::post::model::Gen;
use form_data::{Field, Form};

fn main() {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    let form = Form::new()
    .field("author", Field::text())
    .field("photo", Field::text())
    .field("description", Field::text())
    .field("files", Field::file(Gen));
    
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(establish_connection())
            .data(form.clone())
            .configure(post::router::config)
    })  
        .bind("0.0.0.0:8001")
        .expect("Can not bind to port 8000")
        .run()
        .unwrap();
}
