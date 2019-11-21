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

        new_post.validate().map_err(|e| e);

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
    pub photo: Uuid
}



impl NewPost {
    pub fn post(&self, connection: &PgConnection) -> Result<Post, PostError> {
            use diesel::RunQueryDsl;

            self.validate().map_err(|e| e);
   
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
                .map_err(|e| error::ErrorInternalServerError(e)),
        )
}

use std::env;

use rusoto_core::credential::{AwsCredentials, StaticProvider};
use rusoto_s3::{GetObjectRequest, PutObjectRequest, S3Client, S3,
};


pub fn put_file_s3(srcFile: String, destFile: String) -> impl Future<Item = rusoto_s3::PutObjectOutput, Error = Error> {
    let region =  rusoto_core::region::Region::Custom {
        name: env::var("region").expect("region must be set"),
        endpoint: env::var("endpoint").expect("endpoint must be set"),
    };

    let credentials = AwsCredentials::new(
        env::var("accesskey").expect("accesskey must be set"), 
        env::var("secretkey").expect("secretkey must be set"), 
        None,
        None
    );

    let client = S3Client::new_with(
        rusoto_core::request::HttpClient::new().expect("Failed to creat HTTP client"),
        StaticProvider::from(credentials), 
        region
    );

    web::block(move || {  
        let file= File::open(&srcFile[..]).unwrap();
        let data: Result<Vec<_>, _> = file.bytes().collect();
        let data = data.expect("Unable to read data");

        let put_request = PutObjectRequest {
            bucket: env::var("nameBucket").expect("nameBucket must be set"),
            key: destFile,
            body: Some(data.into()),
            ..Default::default()
        };

        client
            .put_object(put_request)
            .sync()
            .map(|res| res)
            .map_err(|e| e)
    })
    .map(| res | res)
    .map_err(|e: error::BlockingError<rusoto_core::RusotoError<rusoto_s3::PutObjectError>>| {
        match e {
            error::BlockingError::Error(e) => error::ErrorInternalServerError(e),
            error::BlockingError::Canceled => error::ErrorInternalServerError(MultipartError::Incomplete),
        }
    })
}

pub fn get_file_s3(file_path: String) -> Result<String, PostError> {
        let region =  rusoto_core::region::Region::Custom {
            name: env::var("region").expect("region must be set"),
            endpoint: env::var("endpoint").expect("endpoint must be set"),
        };

        let credentials = AwsCredentials::new(
            env::var("accesskey").expect("accesskey must be set"), 
            env::var("secretkey").expect("secretkey must be set"), 
            None,
            None
        );

        let client = S3Client::new_with(
            rusoto_core::request::HttpClient::new().expect("Failed to creat HTTP client"),
            StaticProvider::from(credentials), 
            region
        );

        let get_req = GetObjectRequest {
            bucket: env::var("nameBucket").expect("nameBucket must be set"),
            key: file_path,
            ..Default::default()
        };

        match client
            .get_object(get_req)
            .sync(){
                Ok(res) => {
                    let mut stream = res.body.unwrap().into_blocking_read();
                    let mut body = Vec::new();
                    stream.read_to_end(&mut body).unwrap();
            
                    Ok(base64::encode(&body))
                },
                Err(e) => return Err(PostError::S3GetError(e))
            }
}
use std::path::PathBuf;
use form_data::FilenameGenerator;

#[derive(Debug)]
pub struct Gen;
 
 impl FilenameGenerator for Gen {
     fn next_filename(&self, _: &mime::Mime) -> Option<PathBuf> {
         let mut p = PathBuf::new();
         p.push(format!("{}.png", Uuid::new_v4()));
         Some(p)
     }
}