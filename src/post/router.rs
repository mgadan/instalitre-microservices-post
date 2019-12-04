use actix_web::{web};
use crate::post::handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::post().to(handler::upload))
    );

    cfg.service(
        web::resource("/{id}")
            .route(web::get().to_async(handler::get))
            .route(web::delete().to_async(handler::delete))
            .route(web::put().to_async(handler::put))
    );
    
    cfg.service(
        web::resource("/user/{id}")
            .route(web::get().to(handler::get_all))
            .route(web::delete().to(handler::delete_all))
    );

    cfg.service(
        web::resource("/file/{author}/{post}")
            .route(web::get().to(handler::get_file))
    );
}