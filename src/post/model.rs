use validator::Validate;

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::schema::posts;
use diesel::PgConnection;
use crate::errors::PostError;
use prometheus::{Opts, Registry, Counter, TextEncoder, Encoder};

#[derive(Debug, Validate, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="posts"]
    pub struct Post {
    pub uuid: Uuid,
    pub author: Uuid,
    #[validate(length(min = 1, max = 1000))]
    pub description: String,
    pub photo: Uuid                                                                                                                                                         
}
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             
#[derive(Insertable, Deserialize, AsChangeset)]                                                                                                                                                                                                     
#[table_name="posts"]
pub struct UpdatePost {
    pub description: Option<String>,
}

impl Post {
    pub fn find(uuid: &Uuid, connection: &PgConnection) -> Result<Post, PostError> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        posts::table.find(uuid).first(connection)
    }

    pub fn destroy(uuid: &Uuid, connection: &PgConnection) -> Result<(), PostError> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::posts::dsl;
        diesel::delete(dsl::posts.find(uuid)).execute(connection)?;
        Ok(())
    }

     pub fn update(uuid: &Uuid, new_post: &UpdatePost, connection: &PgConnection) -> Result<(), PostError> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::posts::dsl;
        diesel::update(dsl::posts.find(uuid))
            .set(new_post)
            .execute(connection)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize)] 
pub struct PostList(pub Vec<Post>);

impl PostList {
    pub fn list(connection: &PgConnection) -> Self {
        // These four statements can be placed in the top, or here, your call.
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::posts::dsl::*;
        let result = 
            posts
                .limit(10)
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
    pub fn create(&self, connection: &PgConnection) -> Result<Post, PostError> {
        use diesel::RunQueryDsl;
        println!("{:?}", self);
        // match self.validate() {
        //      Ok(_) => {},
        //      Err(e) => Err(PostError),
        // };
        let post = self.clone();
        let new_post = Post {
            uuid: Uuid::new_v4(),
            photo: Uuid::new_v4(),
            description: post.description,
            author: post.author,
        };
        diesel::insert_into(posts::table)
            .values(new_post)
            .get_result(connection)
    }
}

