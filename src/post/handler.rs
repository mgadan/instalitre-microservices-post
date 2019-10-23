extern crate diesel;
use actix_web::{HttpRequest, HttpResponse };
use crate::post::model::*;
use actix_web::web;
use uuid::Uuid;
use crate::lib::{ PgPool, PgPooledConnection };

fn pg_pool_handler(pool: web::Data<PgPool>) -> Result<PgPooledConnection, HttpResponse> {
    pool
    .get()
    .map_err(|e| {
        HttpResponse::InternalServerError().json(e.to_string())
    })
}

pub fn get_posts(_req: HttpRequest, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Ok(HttpResponse::Ok().json(PostList::list(&pg_pool)))
}

pub fn create_posts(new_post: web::Json<NewPost>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    new_post
        .create(&pg_pool)
        .map(|post| HttpResponse::Ok().json(post))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub fn show(uuid: web::Path<Uuid>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Post::find(&uuid, &pg_pool)
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub fn destroy(uuid: web::Path<Uuid>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Post::destroy(&uuid, &pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub fn update(uuid: web::Path<Uuid>, new_post: web::Json<UpdatePost>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Post::update(&uuid, &new_post, &pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}
