#[cfg(test)]
#[cfg(test)]
mod unitTest {
    use mocktopus::macros::*;

    use ::post::model::{ Post, NewPost };
    use uuid::Uuid;

    #[test]
    fn test(){
        let whisky = NewPost {
            author: Uuid::new_v4(),
            description: "whisky",
            photo: ""
        };
        whisky.
    }
}