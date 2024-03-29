use validator::Validate;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::schema::posts;
use diesel::PgConnection;
use crate::errors::PostError;
use crate::post::models::s3::delete_file_s3;
use actix_web::{error, Error};

#[derive(Debug, Validate, Serialize, Deserialize, Queryable, Insertable, PartialEq, Clone)]
#[table_name="posts"]
    pub struct Post {
    pub id: Uuid,
    pub author: Uuid,
    pub description: String,
    pub photo: Uuid,
    pub created_at: chrono::NaiveDateTime,                                                                                                                                                         
}
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             
#[derive(Insertable, Deserialize, AsChangeset, Validate)]                                                                                                                                                                                                     
#[table_name="posts"]
pub struct UpdatePost {
    #[validate(length(min = 1, max = 1000))]
    pub description: Option<String>,
}

impl Post {
    pub fn get(id_post: &Uuid, connection: &PgConnection) -> Result<Post, PostError> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema;

        let post = schema::posts::table
            .find(id_post)
            .first(connection)?;

        Ok(post)
    }

    pub fn delete(id_post: &Uuid, connection: &PgConnection) -> Result<(), PostError> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema;
        use crate::schema::posts::dsl::*;

        let post: Post = schema::posts::table
            .find(id_post)
            .first(connection)?;   

        diesel::delete(posts.find(id_post))
            .execute(connection)?;

        delete_file_s3(format!("{}/{}.png", post.author, post.photo))?;

        Ok(())
    }

    fn delete_self(self, connection: &PgConnection) -> Result<Post, PostError> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::posts::dsl;

        diesel::delete(dsl::posts.find(self.id))
            .execute(connection)?;
        Ok(self)
    }

     pub fn put(id: &Uuid, new_post: &UpdatePost, connection: &PgConnection) -> Result<(), PostError> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::posts::dsl;
        
        new_post.validate()?;

        diesel::update(dsl::posts.find(id))
            .set(new_post)
            .execute(connection)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize)] 
pub struct PostList(pub Vec<Post>);

impl PostList {
    pub fn get_all(param_author: &Uuid, connection: &PgConnection) -> Result<Self, PostError> {
        use crate::schema::posts::dsl::*;
        use diesel::ExpressionMethods;
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        let result = 
            posts
                .filter(author.eq(param_author))
                .order_by(created_at.desc())
                .load::<Post>(connection)?;

        Ok(PostList(result))
    }

    pub fn delete_all(param_author: &Uuid, connection: &PgConnection) -> Result<(), PostError> {
        use crate::schema::posts::dsl::*;
        use diesel::ExpressionMethods;
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        let result = 
        posts
            .filter(author.eq(param_author))
            .load::<Post>(connection)
            .expect("Error loading post");

        for post in result {
            match post.clone().delete_self(connection) {
                Ok(post)=>{
                    match delete_file_s3(format!("{}/{}.png", post.author, post.photo)){
                        Ok(_)=>(),
                        Err(e)=>return Err(e),
                    }
                },
                Err(e)=>return Err(e),
            }
        }

        Ok(())
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
    pub fn post(&self, connection: &PgConnection) ->  Result<Post, Error> {
        use diesel::RunQueryDsl;
        use diesel::dsl;
        use diesel::prelude::*;
        let post = self.clone();
        match self.validate() {
            Ok(_)=>(),
            Err(e)=> {
                return Err(error::ErrorInternalServerError(e.to_string()))
             },               
         };

        let register = match diesel::insert_into(posts::table)
            .values((
                posts::id.eq(Uuid::new_v4()),
                posts::photo.eq(post.photo),
                posts::description.eq(post.description),
                posts::author.eq(post.author),
                posts::created_at.eq(dsl::now),
            ))           
            .get_result::<Post>(connection) {
                Ok(post)=>post,
                Err(e)=> {
                    return Err(error::ErrorInternalServerError(e.to_string()))
                },
            };
        Ok(register)
    }
}