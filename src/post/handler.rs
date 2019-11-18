extern crate diesel;
use actix_web::{HttpRequest, HttpResponse, Error, error};
use crate::post::model::*;
use actix_web::web;
use uuid::Uuid;
use crate::lib::{ PgPool, PgPooledConnection };
use actix_multipart::Multipart;
use futures::{Future, Stream};


macro_rules! function_handler {
    ( $handler_name:ident ($($arg:ident:$typ:ty),*) -> $body:expr) => {
        pub fn $handler_name(user: LoggedUser, pool: web::Data<PgPool>, $($arg:$typ,)*) 
            -> impl Future<Item = HttpResponse, Error = actix_web::Error>
        {
            web::block(move || {
                let pg_pool = pool
                    .get()
                    .map_err(|_| {
                        crate::errors::MyStoreError::PGConnectionError
                    })?;
                $body(user, pg_pool)
            })
            .then(|res| match res {
                Ok(data) => Ok(HttpResponse::Ok().json(data)),
                Err(error) => Err(actix_web::error::ErrorInternalServerError(error)),
            })
        }
    };
}


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
    author: web::Path<Uuid>,
    multipart: Multipart,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let destFile = format!("./{}/{}.png", author, Uuid::new_v4());
    let srcFile = format!("./{}.png", author);
    multipart
        .map_err(error::ErrorInternalServerError)
        .map(move | field | save_file(&author, field).into_stream())
        .flatten()
        .collect()
        .map(move | _ | put_file_s3(srcFile, destFile))
        .map(| _ | HttpResponse::Ok().json(format!("yes")))
        .map_err(|e| {
            println!("failed: {}", e);
            e
        })
         

}
