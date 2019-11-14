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

 pub fn getAll(_req: HttpRequest, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
     let pg_pool = pg_pool_handler(pool)?;
     Ok(HttpResponse::Ok().json(PostList::getAll(&pg_pool)))
 }

pub fn post(new_post: web::Json<NewPost>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    new_post
        .post(&pg_pool)
        .map(|post| HttpResponse::Ok().json(post))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub fn get(uuid: web::Path<Uuid>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Post::get(&uuid, &pg_pool)
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub fn delete(uuid: web::Path<Uuid>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Post::delete(&uuid, &pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub fn put(uuid: web::Path<Uuid>, new_post: web::Json<UpdatePost>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Post::put(&uuid, &new_post, &pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}
