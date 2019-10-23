extern crate diesel;
use actix_web::{HttpRequest, HttpResponse };
use crate::post::model::*;
use actix_web::web;

pub fn get_posts(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(PostList::list())
}

pub fn create_posts(new_post: web::Json<NewPost>) -> Result<HttpResponse, HttpResponse> {

    // we call the method create from NewProduct and map an ok status response when
    // everything works, but map the error from diesel error 
    // to an internal server error when something fails.
    new_post
        .create()
        .map(|post| HttpResponse::Ok().json(post))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}