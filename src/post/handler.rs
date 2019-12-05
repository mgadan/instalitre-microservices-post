extern crate diesel;
use actix_web::{HttpRequest, HttpResponse, error};
use crate::post::models::post::*; 
use crate::post::models::s3::*; 
use actix_web::web;
use uuid::Uuid;
use crate::db_connection::{ PgPool, PgPooledConnection };
use actix_multipart::Multipart;
use futures::{Future, Stream};
use futures::future;
use actix_multipart::{Field, MultipartError};
use actix_web::{Error};

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
     PostList::get_all(&author, &pg_pool)
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
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


pub fn upload(multipart: Multipart, pool: web::Data<PgPool>) -> impl Future<Item = HttpResponse, Error = HttpResponse> {
    multipart
            .map_err(error::ErrorInternalServerError)
            .map(|field| get_field_filedata(field).into_stream())
            .flatten()
            .collect()
            .map(move |res| {
                let file_bytes = res.first().unwrap().to_vec();
                let author =  String::from_utf8(res[1].to_vec()).unwrap();
                let description =  String::from_utf8(res[2].to_vec()).unwrap();
                let photo = Uuid::new_v4();
                match put_file_s3(file_bytes, format!("{}/{}.png", author, photo)) {
                    Ok(_)=>(),
                    Err(e)=> return HttpResponse::InternalServerError().json(e.to_string()),
                };
                let pg_pool = match pg_pool_handler(pool){
                    Ok(db_connection)=>db_connection,
                    Err(e)=> return e,
                };
                println!("{}", author);
                println!("{}", photo);
                println!("{}", description);

                let new_post = NewPost{
                    author: Uuid::parse_str(&author[..]).unwrap(),
                    description: description,
                    photo: photo
                };

                match new_post.post(&pg_pool) {
                    Ok(_)=> HttpResponse::Created().finish(),
                    Err(e)=> HttpResponse::InternalServerError().json(e.to_string())
                }    
            })
            .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn get_file(uuid: web::Path<(Uuid, Uuid)>) -> Result<HttpResponse, HttpResponse> {
    get_file_s3(format!("{}/{}.png", uuid.0, uuid.1))
        .map(|res| HttpResponse::Ok().body(res))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn get_field_filedata(field: Field) -> impl Future<Item = Vec<u8>, Error = Error>
{

        field.fold(Vec::new(), move |mut acc : Vec<u8>, bytes| {
            acc.append(bytes.to_vec().as_mut());
            let rt: Result<Vec<u8>, MultipartError> = Ok(acc);
            future::result(rt)
        })
        .map_err(|e| {
            println!("bytes receive failed, {:?}", e);
            error::ErrorInternalServerError(e)
        })
}
