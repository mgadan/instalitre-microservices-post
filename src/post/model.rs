use validator::Validate;

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::schema::posts;
use diesel::PgConnection;
use crate::errors::PostError;
use actix_multipart::{Field, MultipartError};
use actix_web::{error, web, Error};
use futures::future::{err, Either};
use futures::{Future, Stream};
use std::fs;
use std::io::Write;
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug, Validate, Serialize, Deserialize, Queryable, Insertable, PartialEq)]
#[table_name="posts"]
    pub struct Post {
    pub id: Uuid,
    pub author: Uuid,
    pub description: String,
    pub photo: String                                                                                                                                                         
}
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             
#[derive(Insertable, Deserialize, AsChangeset, Validate)]                                                                                                                                                                                                     
#[table_name="posts"]
pub struct UpdatePost {
    #[validate(length(min = 1, max = 1000))]
    pub description: Option<String>,
}

impl Post {
    pub fn get(_id: &Uuid, connection: &PgConnection) -> Result<Post, PostError> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::posts::dsl::*;
        use crate::schema;

        let post = schema::posts::table
                    .find(id)
                    .first(connection)?;

        Ok(post)
    }

    pub fn delete(id: &Uuid, connection: &PgConnection) -> Result<(), PostError> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::posts::dsl;

        diesel::delete(dsl::posts.find(id))
            .execute(connection)?;
        Ok(())
    }

     pub fn put(id: &Uuid, new_post: &UpdatePost, connection: &PgConnection) -> Result<(), PostError> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::posts::dsl;

        match new_post.validate() {
            Ok(_) => {},
            Err(e) => {
               return Err(PostError::ValidatorInvalid(e));
           }
       };

        diesel::update(dsl::posts.find(id))
            .set(new_post)
            .execute(connection)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize)] 
pub struct PostList(pub Vec<Post>);

impl PostList {
    pub fn get_all(connection: &PgConnection) -> Self{
        // These four statements can be placed in the top, or here, your call.
        use diesel::RunQueryDsl;
        use crate::schema::posts::dsl::*;

        let result = 
            posts
                .load::<Post>(connection)
                .expect("Error loading post");

        // We return a value by leaving it without a comma
        PostList(result)
    }
}

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct NewPost {
    pub author:  Uuid,
    #[validate(length(min = 1, max = 1000))]
    pub description: String,
    #[validate(contains = "data:image/jpg;base64")]
    pub photo: String
}



impl NewPost {
    pub fn post(&self, connection: &PgConnection) -> Result<Post, PostError> {
        use diesel::RunQueryDsl;
        println!("{:?}", self);
        
        match self.validate() {
             Ok(_) => {},
             Err(e) => {
                return Err(PostError::ValidatorInvalid(e));
            }
        };

        let post = self.clone();
        let new_post = Post {
            id: Uuid::new_v4(),
            photo: format!("{}", Uuid::new_v4()),
            description: post.description,
            author: post.author,
        };
        
        let register = diesel::insert_into(posts::table)
        .values(new_post)
        .get_result::<Post>(connection)?;

        Ok(register)
    }
}

pub fn save_file(id: &Uuid, field: Field) -> impl Future<Item = u16, Error = Error> {
    let validator_images = format!("{:?}", field.headers().get("content-type"));
    println!("{:?}", field.headers());
    if validator_images.contains("image") != true {
        return Either::A(err(error::ErrorInternalServerError(std::io::Error::new(std::io::ErrorKind::Other, "Votre fichier n'est pas une images"))));
    }
    //TODO : remplacer la valeur static par la valeur dynamique
    let file_path_string = (format!("./{}.png", id)).to_owned();

    let file = match fs::File::create(file_path_string) {
            Ok(file) => file,
            Err(e) => return Either::A(err(error::ErrorInternalServerError(e))),
        };
        Either::B(
            field
                .fold((file, 0i64), move |(mut file, mut acc), bytes| {
                    web::block(move || {
                        file.write_all(bytes.as_ref()).map_err(|e| {
                            println!("file.write_all failed: {:?}", e);
                            MultipartError::Payload(error::PayloadError::Io(e))
                        })?;
                        acc += bytes.len() as i64;
                        Ok((file, acc))
                    })
                    .map_err(|e: error::BlockingError<MultipartError>| {
                        match e {
                            error::BlockingError::Error(e) => e,
                            error::BlockingError::Canceled => MultipartError::Incomplete,
                        }
                    })
                })
                .map(| _ | 200)
                .map_err(|e| {
                    println!("save_file failed, {:?}", e);
                    error::ErrorInternalServerError(e)
                }),
        )
}

use s3::bucket::Bucket;
use s3::credentials::Credentials;
use s3::region::Region;


pub fn put_file_s3(path: String) -> impl Future<Item = u16, Error = Error> {
    let region = Region::Custom {
        region: format!("fr-par"),
        endpoint: format!("https://s3.fr-par.scw.cloud")
    };

    let credentials = Credentials::new(
        Some(format!("SCWTRK0PCJK8Z9626DGF")), 
        Some(format!("6b9e1031-1c23-4f49-abdb-be90131752d6")), 
        None, 
    None);

    let bucket = Bucket::new(
        &format!("s3cloud"),
        region,
        credentials
    ).unwrap();

    web::block(move || {  
        let s_slice: &str = &path[..];  // take a full slice of the string
        let file= File::open(s_slice).unwrap();
        let data: Result<Vec<_>, _> = file.bytes().collect();
        let data = data.expect("Unable to read data");
        let (_, code) = bucket.put_object(s_slice, &data, "image/png").unwrap();
        fs::remove_file(s_slice);
        println!("{:?}", code);
        Ok(code)
    })
    .map(| code | code)
    .map_err(|e: error::BlockingError<MultipartError>| {
        match e {
            error::BlockingError::Error(e) => error::ErrorInternalServerError(e),
            error::BlockingError::Canceled => error::ErrorInternalServerError(MultipartError::Incomplete),
        }
    })
}