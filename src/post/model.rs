use validator::Validate;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
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