use uuid::Uuid;
use crate::errors::PostError;
use actix_multipart::{Field, MultipartError};
use actix_web::{error, web, Error};
use futures::{
    future::{err, Either}, 
    Future, Stream
};
use std::fs;
use std::io::{
    Write,
    prelude::*
};
use std::fs::File;
use std::env;
use rusoto_core::credential::{AwsCredentials, StaticProvider};
use rusoto_s3::{DeleteObjectRequest, GetObjectRequest, PutObjectRequest, S3Client, S3,};
use std::path::PathBuf;
use form_data::{ FilenameGenerator, Value};
use regex::Regex;
use crate::post::models::post::NewPost; 

pub fn save_file(id: &Uuid, field: Field) -> impl Future<Item = u16, Error = Error> {
    let validator_images = format!("{:?}", field.headers().get("content-type"));
    println!("{:?}", field.headers());
    if validator_images.contains("image") != true {
        return Either::A(err(error::ErrorInternalServerError(std::io::Error::new(std::io::ErrorKind::Other, "Votre fichier n'est pas une images"))));
    }
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

fn get_region() -> Result<rusoto_signature::Region, PostError> {
    let region_name = match env::var("region"){
        Ok(name)=>name,
        Err(e)=> return Err(PostError::InvalidEnv(e)),  
      };

    let region_endpoint = match env::var("endpoint"){
        Ok(name)=>name,
        Err(e)=> return Err(PostError::InvalidEnv(e)),  
      };

    let region =  rusoto_core::region::Region::Custom {
        name: region_name,
        endpoint: region_endpoint,
    };
    Ok(region)
}

fn get_credentials() -> Result<rusoto_credential::AwsCredentials, PostError> {
    let access_key = match env::var("accesskey"){
        Ok(value)=>value,
        Err(e)=> return Err(PostError::InvalidEnv(e)),  
      };

    let secret_key = match env::var("secretkey"){
        Ok(value)=>value,
        Err(e)=> return Err(PostError::InvalidEnv(e)),  
      };

    let credentials = AwsCredentials::new(
        access_key, 
        secret_key, 
        None,
        None
    );
    Ok(credentials)
}

fn get_client() -> Result<rusoto_s3::S3Client, PostError> {
    let region = match get_region() {
        Ok(value)=>value,
        Err(e)=>return Err(e),
    };

    let credentials = match get_credentials() {
        Ok(value)=>value,
        Err(e)=>return Err(e),
    };

    let client = S3Client::new_with(
        rusoto_core::request::HttpClient::new().expect("Failed to creat HTTP client"),
        StaticProvider::from(credentials), 
        region
    );
    Ok(client)
}

pub fn put_file_s3(src_file: String, dest_file: String) -> Result<(), PostError> {
        //read data
        let file= File::open(&src_file[..]).unwrap();
        let data: Result<Vec<_>, _> = file.bytes().collect();
        let data = match data {
            Ok(data)=> data,
            Err(e) => return Err(PostError::InvalidReadFile(e)),
        };

        //initialise aws
        let name_bucket = match env::var("nameBucket"){
            Ok(name)=>name,
            Err(e)=> return Err(PostError::InvalidEnv(e)),  
          };

        let client = match get_client() {
            Ok(value)=>value,
            Err(e)=>return Err(e),
        };

        //request
        let put_request = PutObjectRequest {
            bucket: name_bucket,
            key: dest_file,
            body: Some(data.into()),
            ..Default::default()
        };

        match client
            .put_object(put_request)
            .sync() {
                Ok(_)=> {
                    fs::remove_file(src_file.clone()).expect("fichier n'existe pas");
                    Ok(())
                },
                Err(e)=> {
                    fs::remove_file(src_file.clone()).expect("fichier n'existe pas");
                    return Err(PostError::S3PutError(e))
                }
            }
}
    
pub fn get_file_s3(file_path: String) -> Result<String, PostError> {
        let client = match get_client() {
            Ok(value)=>value,
            Err(e)=>return Err(e),
        };

        let name_bucket = match env::var("nameBucket"){
            Ok(name)=>name,
            Err(e)=> return Err(PostError::InvalidEnv(e)),  
          };

        let get_req = GetObjectRequest {
            bucket: name_bucket,
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


pub fn delete_file_s3(file_path: String) -> Result<(), PostError> {
    let client = match get_client() {
        Ok(value)=>value,
        Err(e)=>return Err(e),
    };

    let name_bucket = match env::var("nameBucket"){
        Ok(name)=>name,
        Err(e)=> return Err(PostError::InvalidEnv(e)),  
      };

    let delete_req = DeleteObjectRequest {
        bucket: name_bucket,
        key: file_path,
        ..Default::default()
    };
    match client
        .delete_object(delete_req)
        .sync(){
            Ok(_) => Ok(()),
            Err(e) => return Err(PostError::S3DeleteError(e))
        }
}
#[derive(Debug)]
pub struct Gen;
 
 impl FilenameGenerator for Gen {
     fn next_filename(&self, _: &mime::Mime) -> Option<PathBuf> {
         let mut p = PathBuf::new();
         p.push(format!("{}.png", Uuid::new_v4()));
         Some(p)
     }
}

pub fn form_data_value_to_new_post(uploaded_content: Value) -> NewPost {

    let mut photo_post = format!("");
    let mut author_post = format!("");
    let mut description_post = format!("");

    match uploaded_content {
        Value::Map(hashmap) => {
            match hashmap.get("author") {
                Some(value) => match value {
                    Value::Text(text) => author_post = text.to_uppercase(),
                    _ => (),
                }
                None => (),
            }
            match hashmap.get("description") {
                Some(value) => match value {
                    Value::Text(text) => description_post = format!("{}", text),
                    _ => (),
                }
                None => (),
            }
            match hashmap.get("files") {
                Some(value) => match value {
                    Value::File(_, path_buf) => photo_post = format!("{:?}", path_buf),
                    _ => (),
                }
                None => (),
            }
        }
        _ => (),
    }
    let regex = Regex::new(r"(?m)[0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}").unwrap();    
    let caps = regex.captures(&photo_post[..]).unwrap();
    let new_post = NewPost {
        photo: Uuid::parse_str(&caps.get(0).unwrap().as_str()[..]).unwrap(),
        description: description_post,
        author: Uuid::parse_str(&author_post[..]).unwrap(),
    };
    return new_post;
}
