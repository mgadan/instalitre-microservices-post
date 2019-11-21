use actix_web::{web};
use crate::post::handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to_async(handler::get_all))
            //.route(web::post().to_async(handler::post))
    );

    cfg.service(
        web::resource("/{id}")
            .route(web::get().to_async(handler::get))
            .route(web::delete().to_async(handler::delete))
            .route(web::patch().to_async(handler::put))
    );

    cfg.service(
        web::resource("/file/{author}/{post}")
            .route(web::post().to_async(handler::upload))
            .route(web::get().to_async(handler::get_file))
    );

    cfg.service(
        web::resource("/test/")
            .route(web::post().to(handler::upload2))
        );
}
