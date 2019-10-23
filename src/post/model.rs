use validator::Validate;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::schema::posts;


#[derive(Debug, Validate, Deserialize, Serialize, Queryable)]
    pub struct Post {
    //#[validate(length(equal = 36))]
    pub id: Uuid,
    //#[validate(length(equal = 36))]
    pub author: Uuid,
    #[validate(length(min = 1, max = 1000))]
    pub description: String,
    //#[validate(contains = "data:image/jpg;base64")]
    pub photo: Uuid
}

#[derive(Serialize, Deserialize)] 
pub struct PostList(pub Vec<Post>);

impl PostList {
    pub fn list() -> Self {
        // These four statements can be placed in the top, or here, your call.
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use post::establish_connection;
        use crate::schema::posts::dsl::*;
        
        let connection = establish_connection();

        let result = 
            posts
                .limit(10)
                .load::<Post>(&connection)
                .expect("Error loading post");

        // We return a value by leaving it without a comma
        PostList(result)
    }
}

#[derive(Insertable, Deserialize, Debug, Clone)]
#[table_name="posts"]
pub struct NewPost {
    pub uuid: Option<Uuid>,
    pub author: Uuid,
    pub description: String,
    pub photo: Uuid
}

impl NewPost {

    // Take a look at the method definition, I'm borrowing self, 
    // just for fun remove the & after writing the handler and 
    // take a look at the error, to make it work we would need to use into_inner (https://actix.rs/api/actix-web/stable/actix_web/struct.Json.html#method.into_inner)
    // which points to the inner value of the Json request.
    pub fn create(&self) -> Result<Post, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use post::establish_connection;

        let new_post = NewPost {
            uuid: Some( Uuid::new_v4()),
            ..self.clone()
        };

        let connection = establish_connection();
        diesel::insert_into(posts::table)
            .values(new_post)
            .get_result(&connection)
    }
}


