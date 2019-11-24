extern crate diesel;
use actix_web::{HttpRequest, HttpResponse, error};
use actix_web::web::Data;
use crate::post::models::post::*; 
use crate::post::models::s3::*; 
use actix_web::web;
use uuid::Uuid;
use crate::db_connection::{ PgPool, PgPooledConnection };
use actix_multipart::Multipart;
use futures::{Future, Stream};
use form_data::{handle_multipart, Form};

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

 pub fn get_all(_req: HttpRequest, author: web::Path<Uuid>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
     let pg_pool = pg_pool_handler(pool)?;
     Ok(HttpResponse::Ok().json(PostList::get_all(&author, &pg_pool)))
 }

 pub fn delete_all(id: web::Path<Uuid>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    PostList::delete_all(&id, &pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

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

pub fn upload((mp, state, pool): (Multipart, Data<Form>, web::Data<PgPool>)) -> Box<dyn Future<Item = HttpResponse, Error = HttpResponse>> {
    let pg_pool = pg_pool_handler(pool).expect("la connexion a échouée");
    Box::new(
        handle_multipart(mp, state.get_ref().clone())
        .map(|uploaded_content| form_data_value_to_new_post(uploaded_content))
        .map(move |new_post| new_post.post(&pg_pool))
        .map(| post | {
            println!("{:?}", post);
            match post {
                Ok(post) => {

                    Ok(())
                },
                Err(e) => {
                    println!("{}", e.to_string());
                    return Err(e);
                },
            }
        })
        .map(| _ | HttpResponse::Created().finish())
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string())),
    )
}

pub fn get_file(uuid: web::Path<(Uuid, Uuid)>) -> Result<HttpResponse, HttpResponse> {
    get_file_s3(format!("{}/{}.png", uuid.0, uuid.1))
        .map(|res| HttpResponse::Ok().body(res))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}