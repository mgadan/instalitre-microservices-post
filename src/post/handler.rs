extern crate diesel;
use actix_web::{HttpRequest, HttpResponse, Error, error};
use crate::post::model::*;
use actix_web::web;
use uuid::Uuid;
use crate::lib::{ PgPool, PgPooledConnection };
use actix_multipart::Multipart;
use futures::{Future, Stream};

fn pg_pool_handler(pool: web::Data<PgPool>) -> Result<PgPooledConnection, HttpResponse> {
    pool
    .get()
    .map_err(|e| {
        HttpResponse::InternalServerError().json(e.to_string())
    })
}

 pub fn get_all(_req: HttpRequest, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
     let pg_pool = pg_pool_handler(pool)?;
     Ok(HttpResponse::Ok().json(PostList::get_all(&pg_pool)))
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

pub fn get(id: web::Path<Uuid>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Post::get(&id, &pg_pool)
        .map(|post| HttpResponse::Ok().json(post))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub fn delete(id: web::Path<Uuid>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Post::delete(&id, &pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub fn put(id: web::Path<Uuid>, new_post: web::Json<UpdatePost>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Post::put(&id, &new_post, &pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub fn upload(
    id: web::Path<Uuid>,
    multipart: Multipart,
) -> impl Future<Item = HttpResponse, Error = Error> {
    multipart
        .map_err(error::ErrorInternalServerError)
        .map(| field | save_file(field).into_stream())
        .flatten()
        .collect()
        .map(|sizes| HttpResponse::Ok().json(sizes))
        .map_err(|e| {
            println!("failed: {}", e);
            e
        })

}
