extern crate openssl;
#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate diesel;

pub mod schema;
pub mod post;
pub mod db_connection;
pub mod errors;

use db_connection::establish_connection;
use actix_web::{middleware, http::header, App, HttpServer};
use actix_cors::Cors;
use crate::post::models::s3::Gen;
use form_data::{Field, Form};
use std::env;
use dotenv::dotenv;

fn main() {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    dotenv().ok();
    let port = env::var("PORT").expect("port must be set");

    let form = Form::new()
        .field("author", Field::text())
        .field("description", Field::text())
        .field("files", Field::file(Gen));
    
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin(&env::var("ALLOWED_ORIGIN").expect("not exist variable ALLOWED_ORIGIN")[..])
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION,
                                        header::CONTENT_TYPE,
                                        header::ACCEPT])
                    .max_age(3600)
            )
            .wrap(middleware::Logger::default())
            .data(establish_connection())
            .data(form.clone())
            .configure(post::router::config)
    })  
        .bind(format!("0.0.0.0:{}", port))
        .expect("Can not bind to port 8000")
        .run()
        .unwrap();
}
