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

pub fn save_file( field: Field) -> impl Future<Item = i64, Error = Error> {
    let file_path_string = format!("upload.png");
    let file = match fs::File::create(file_path_string) {
        Ok(file) => file,
        Err(e) => return Either::A(err(error::ErrorInternalServerError(e))),
    };
    Either::B(
        field
            .fold((file, 0i64), move |(mut file, mut acc), bytes| {
                // fs operations are blocking, we have to execute writes
                // on threadpool
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
            .map(|(_, acc)| acc)
            .map_err(|e| {
                println!("save_file failed, {:?}", e);
                error::ErrorInternalServerError(e)
            }),
    )
}