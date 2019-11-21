extern crate diesel;
use actix_web::{HttpRequest, HttpResponse, Error, error};
use actix_web::web::Data;
use crate::post::model::*;
use actix_web::web;
use uuid::Uuid;
use crate::lib::{ PgPool, PgPooledConnection };
use actix_multipart::Multipart;
use futures::{Future, Stream};
use form_data::{handle_multipart, Form, Value};

// macro_rules! function_handler {
//     ( $handler_name:ident ($($arg:ident:$typ:ty),*) -> $body:expr) => {
//         pub fn $handler_name(user: LoggedUser, pool: web::Data<PgPool>, $($arg:$typ,)*) 
//             -> impl Future<Item = HttpResponse, Error = actix_web::Error>
//         {
//             web::block(move || {
//                 let pg_pool = pool
//                     .get()
//                     .map_err(|_| {
//                         crate::errors::MyStoreError::PGConnectionError
//                     })?;
//                 $body(user, pg_pool)
//             })
//             .then(|res| match res {
//                 Ok(data) => Ok(HttpResponse::Ok().json(data)),
//                 Err(error) => Err(actix_web::error::ErrorInternalServerError(error)),
//             })
//         }
//     };
// }


fn pg_pool_handler(pool: web::Data<PgPool>) -> Result<PgPooledConnection, HttpResponse> {
    pool
    .get()
    .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

 pub fn get_all(_req: HttpRequest, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
     let pg_pool = pg_pool_handler(pool)?;
     Ok(HttpResponse::Ok().json(PostList::get_all(&pg_pool)))
 }
/*
pub fn post(new_post: web::Json<NewPost>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    new_post
        .post(&pg_pool)
        .map(|post| HttpResponse::Ok().json(post))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}*/

pub fn get(id: web::Path<Uuid>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Post::get(&id, &pg_pool)
        .map(|post| HttpResponse::Ok().json(post))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn delete(id: web::Path<Uuid>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Post::delete(&id, &pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn put(id: web::Path<Uuid>, new_post: web::Json<UpdatePost>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Post::put(&id, &new_post, &pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn upload(
    uuid: web::Path<(Uuid, Uuid)>,
    multipart: Multipart,
) -> impl Future<Item = HttpResponse, Error = HttpResponse> {
    let dest_file = format!("{}/{}.png", uuid.0, uuid.1);
    let uuid_temp = Uuid::new_v4();
    let src_file = format!("./{}.png", uuid_temp);
    multipart
        .map_err(error::ErrorInternalServerError)
        .map(move | field | save_file(&uuid_temp, field).into_stream())
        .flatten()
        .collect()
        .map(move | _ | put_file_s3(src_file, dest_file))
        .flatten()
        .map(| _ | HttpResponse::Created().finish())
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn upload2((mp, state, pool): (Multipart, Data<Form>, web::Data<PgPool>)) -> Box<dyn Future<Item = HttpResponse, Error = HttpResponse>> {
    let pg_pool = pg_pool_handler(pool).expect("la connexion a échouée");
    Box::new(
        handle_multipart(mp, state.get_ref().clone())
        .map_err(error::ErrorInternalServerError)
        .map(|uploaded_content| {
            let mut photo_post = format!("");
            let mut author_post = format!("");
            let mut description_post = format!("");
            match uploaded_content {
                Value::Map(mut hashmap) => {
                    match hashmap.remove("photo") {
                        Some(value) => match value {
                            Value::Text(text) => photo_post = text.to_uppercase(),
                            _ => (),
                        }
                        None => (),
                    }
                    match hashmap.remove("author") {
                        Some(value) => match value {
                            Value::Text(text) => author_post = text.to_uppercase(),
                            _ => (),
                        }
                        None => (),
                    }
                    match hashmap.remove("description") {
                        Some(value) => match value {
                            Value::Text(text) => description_post = text,
                            _ => (),
                        }
                        None => (),
                    }
                }
                _ => (),
            }
            let new_post = NewPost {
                photo: Uuid::parse_str(&photo_post[..]).unwrap(),
                description: description_post,
                author: Uuid::parse_str(&author_post[..]).unwrap(),
            };
            return new_post;
        })
        .map(move |new_post| new_post.post(&pg_pool))
        //.map(| post | put_file_s3(src_file, dest_file))
        .map(| _ | HttpResponse::Created().finish())
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string())),
    )
}

pub fn get_file(uuid: web::Path<(Uuid, Uuid)>) -> Result<HttpResponse, HttpResponse> {
    get_file_s3(format!("{}/{}.png", uuid.0, uuid.1))
        .map(|res| HttpResponse::Ok().body(res))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}