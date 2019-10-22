extern crate diesel;

use actix_web::{HttpRequest, Responder, HttpResponse };
use crate::post::model::PostList;
use post::establish_connection;

pub fn get_posts(req: HttpRequest) -> impl Responder {
    let _connection = establish_connection();

    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name);
}

pub fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(PostList::list())
}

