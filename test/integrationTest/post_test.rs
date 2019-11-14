#[macro_use]
extern crate dotenv_codegen;

mod dbTest;

mod integrationTest {
    use actix_http::HttpService;
    use actix_http_test::{ TestServer, TestServerRuntime };
    use actix_web::http::header;
    use actix_web::{http, App, web};
    use actix_http::httpmessage::HttpMessage;

    use serde_json::json;
    use std::str;
    use std::time::Duration as std_duration;
    use crate::dbTest::db_connection::establish_connection;
    use std::cell::{ RefCell, RefMut };

    use ::post::model::{ Post, NewPost, UpdatePost };
    use uuid::Uuid;

    #[test]
    fn test() {

        let srv = RefCell::new(TestServer::new(move || 
            HttpService::new(
                App::new()
                    .data(establish_connection())
                    .service(
                        web::resource("/")
                            .route(web::get()
                                .to(::post::handlers::index))
                            .route(web::post()
                                .to(::post::handlers::create))
                    )
                    .service(
                        web::resource("/{id}")
                            .route(web::get()
                                .to(::post::handlers::show))
                            .route(web::delete()
                                .to(::post::handlers::destroy))
                            .route(web::patch()
                                .to(::post::handlers::update))
                    )

            )
        ));

        clear_post();

        let whisky = NewPost {
            author: Uuid::new_v4(),
            description: "whisky",
            photo: ""
        };

        let rhum = NewPost {
            author: Uuid::new_v4(),
            description: "rhum",
            photo: ""
        };

        let wine = NewPost {
            author: Uuid::new_v4(),
            description: "wine",
            photo: ""
        };

        let whisky_db = create_a_post(srv.borrow_mut(), &whisky);
        let rhum_db = create_a_post(srv.borrow_mut(), &rhum);
        let wine_db = create_a_post(srv.borrow_mut(), &wine);

        show_a_post(srv.borrow_mut(), &rhum_db.id, &rhum_db);

        let updated_rhum = UpdatePost {
            description: "rhum vide"
        };

        update_a_post(srv.borrow_mut(), 
                         &rhum_db.id, 
                         &updated_rhum);
        destroy_a_post(srv.borrow_mut(), 
                          &wine_db.id);
        posts_index(srv.borrposts_indexow_mut(), 
                       vec![whisky, updated_rhum]);
    }

    fn clear_post() {
        use diesel::RunQueryDsl;
        use ::mystore_lib::schema::products;
        let connection = establish_connection();
        let pg_pool = connection.get().unwrap();
        diesel::delete(post::table).execute(&pg_pool).unwrap();
    }

    fn create_a_post(mut srv: RefMut<TestServerRuntime>, 
                        post: &NewPost) -> Post {

        let request = srv
                          .post("/")
                          .header(header::CONTENT_TYPE, "application/json")
                          .timeout(std_duration::from_secs(600));

        let mut response =
            srv
                .block_on(request.send_body(json!(post).to_string()))
                .unwrap();

        assert!(response.status().is_success());

        let bytes = srv.block_on(response.body()).unwrap();
        let body = str::from_utf8(&bytes).unwrap();
        serde_json::from_str(body).unwrap()
    }

    fn show_a_post(mut srv: RefMut<TestServerRuntime>, 
                        id: Uuid, 
                        expected_post: &Post) {

        let request = srv
                        .get(format!("/{}", id));

        let mut response = srv.block_on(request.send()).unwrap();
        assert!(response.status().is_success());

        assert_eq!(
            response.headers().get(http::header::CONTENT_TYPE).unwrap(),
            "application/json"
        );

        let bytes = srv.block_on(response.body()).unwrap();
        let body = str::from_utf8(&bytes).unwrap();
        let response_post: Post = serde_json::from_str(body).unwrap();
        assert_eq!(&response_post, expected_post);
    }

    fn update_a_post(mut srv: RefMut<TestServerRuntime>,
                            id: Uuid, 
                            changes_to_post: &UpdatePost) {

        let request = srv
                        .request(http::Method::PUT, srv.url(&format!("/{}", id)))
                        .header(header::CONTENT_TYPE, "application/json")
                        .timeout(std_duration::from_secs(600));

        let response =
            srv
                .block_on(request.send_body(json!(changes_to_post).to_string()))
                .unwrap();
        assert!(response.status().is_success());
    }

    fn destroy_a_post(mut srv: RefMut<TestServerRuntime>,
                          id: Uuid) {
        let request = srv
                        .request(http::Method::DELETE, srv.url(&format!("/{}", id)))
                        .header(header::CONTENT_TYPE, "application/json")
                        .timeout(std_duration::from_secs(600));

        let response =
            srv
                .block_on(request.send())
                .unwrap();
        assert!(response.status().is_success());
    }

    fn posts_index(mut srv: RefMut<TestServerRuntime>,
                      mut data_to_compare: Vec<NewPost>) {

        let request = srv
                        .get("/")

        let mut response = srv.block_on(request.send()).unwrap();
        assert!(response.status().is_success());

        assert_eq!(
            response.headers().get(http::header::CONTENT_TYPE).unwrap(),
            "application/json"
        );

        let bytes = srv.block_on(response.body()).unwrap();
        let body = str::from_utf8(&bytes).unwrap();
        let mut response_posts: Vec<Post> = serde_json::from_str(body).unwrap();
        data_to_compare.sort_by_key(|post| post.description.clone());
        response_posts.sort_by_key(|post| post.description.clone());
        assert_eq!(data_to_compare, response_posts);
    }

}