use validator::Validate;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::schema::posts;
use diesel::PgConnection;
use crate::errors::PostError;

#[derive(Debug, Validate, Serialize, Deserialize, Queryable, Insertable, PartialEq)]
#[table_name="posts"]
    pub struct Post {
    pub id: Uuid,
    pub author: Uuid,
    pub description: String,
    pub photo: Uuid                                                                                                                                                         
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
    pub fn get_all(param_author: &Uuid, connection: &PgConnection) -> Self{
        use crate::schema::posts::dsl::*;
        use diesel::ExpressionMethods;
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        let result = 
            posts
                .filter(author.eq(param_author))
                .load::<Post>(connection)
                .expect("Error loading post");

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

            match self.validate(){
                Ok(_)=>(),
                Err(e) => return Err(PostError::ValidatorInvalid(e)),
            }

           let post = self.clone();
           let new_post = Post {
               id: Uuid::new_v4(),
               photo: post.photo,
               description: post.description,
               author: post.author,
           };

           let register = diesel::insert_into(posts::table)
           .values(new_post)
           .get_result::<Post>(connection)?;
           println!("erreur");
           Ok(register)
    }
}