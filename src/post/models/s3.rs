use crate::errors::PostError;
use std::io::prelude::*;
use std::env;
use rusoto_core::credential::{AwsCredentials, StaticProvider};
use rusoto_s3::{DeleteObjectRequest, GetObjectRequest, PutObjectRequest, S3Client, S3,};


fn get_region() -> Result<rusoto_signature::Region, PostError> {
    let region_name = match env::var("REGION"){
        Ok(name)=>name,
        Err(e)=> return Err(PostError::InvalidEnv(e)),  
      };

    let region_endpoint = match env::var("ENDPOINT"){
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
    let access_key = match env::var("ACCESSKEY"){
        Ok(value)=>value,
        Err(e)=> return Err(PostError::InvalidEnv(e)),  
      };

    let secret_key = match env::var("SECRETKEY"){
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

pub fn put_file_s3(data: Vec<u8>, dest_file: String) -> Result<(), PostError> {


        //initialise aws
        let name_bucket = match env::var("NAME_BUCKET"){
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
                Ok(_)=>Ok(()),
                Err(e)=> {
                    return Err(PostError::S3PutError(e))
                }
            }
}
    
pub fn get_file_s3(file_path: String) -> Result<String, PostError> {
        let client = match get_client() {
            Ok(value)=>value,
            Err(e)=>return Err(e),
        };

        let name_bucket = match env::var("NAME_BUCKET"){
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

    let name_bucket = match env::var("NAME_BUCKET"){
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