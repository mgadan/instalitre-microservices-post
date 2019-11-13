use actix_web::{web};
use crate::post::handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to_async(handler::getAll))
            .route(web::post().to_async(handler::post))
    );

    cfg.service(
        web::resource("/{uuid}")
            .route(web::get().to_async(handler::get))
            .route(web::delete().to_async(handler::delete))
            .route(web::patch().to_async(handler::update))
    );
}
