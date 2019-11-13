use validator::Validate;

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::schema::posts;
use diesel::PgConnection;
use crate::errors::PostError;

#[derive(Debug, Validate, Serialize, Deserialize, Queryable, Insertable, PartialEq)]
#[table_name="posts"]
    pub struct Post {
    pub uuid: Uuid,
    pub author: Uuid,
    #[validate(length(min = 1, max = 1000))]
    pub description: String,
    pub photo: Uuid                                                                                                                                                         
}

type PostColumns = (
    posts::uuid,
    posts::description,
    posts::author,
    posts::photo
);

const POST_COLUMNS: PostColumns = (
    posts::uuid,
    posts::description,
    posts::author,
    posts::photo
);

                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             
#[derive(Insertable, Deserialize, AsChangeset)]                                                                                                                                                                                                     
#[table_name="posts"]
pub struct UpdatePost {
    pub description: Option<String>,
}

impl Post {
    pub fn get(uuid: &Uuid, connection: &PgConnection) -> Result<Post, PostError> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use diesel::ExpressionMethods;
        use crate::schema::posts::dsl::*;
        use crate::schema;

        let post: Post =
            schema::posts::table
                .select(POST_COLUMNS)
                .find(uuid)
                .first(connection)?;
        Ok((post))
        // posts::table.find(uuid).first(connection)
    }

    pub fn delete(uuid: &Uuid, connection: &PgConnection) -> Result<(), PostError> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::posts::dsl;
        use diesel::ExpressionMethods;

        diesel::delete(dsl::posts.find(uuid))
            .execute(connection)?;
        Ok(())
    }

     pub fn put(uuid: &Uuid, new_post: &UpdatePost, connection: &PgConnection) -> Result<(), PostError> {
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
    pub fn getAll(connection: &PgConnection) -> Result<Self, PostError> {
        // These four statements can be placed in the top, or here, your call.
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use diesel::ExpressionMethods;
        use crate::schema::posts::dsl::*;
        use diesel::pg::Pg;
        use crate::schema;

        let mut query = schema::posts::table.into_boxed::<Pg>();

        let query_posts = query
            .select(POST_COLUMNS)
            .limit(10)
            .load::<Post>(connection)?;
        // We return a value by leaving it without a comma
        Ok(
            PostList(
                query_posts
                    .into_iter()
                    .zip(query_posts)
                    .collect::<Vec<_>>()
            )
        )
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
        let register = diesel::insert_into(posts::table)
            .values(new_post)
            .get_result::<Post>(connection)?;
        Ok((register))
    }
}

