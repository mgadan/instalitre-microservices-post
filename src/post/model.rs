#[derive(Debug, Validate, Deserialize, Serialize, Queryable)]
pub struct Post {
    #[validate(length(equal = "36"))]
    pub uid: String,
    #[validate(length(equal = "36"))]
    pub author: String,
    #[validate(length(min = "1", max = "1000"))]
    pub description: String,
    #[validate(contains(val = "data:image/jpg;base64"))]
    pub photo: String
}
