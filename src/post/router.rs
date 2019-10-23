use actix_web::{web};
use crate::post::handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to_async(handler::get_posts))
            .route(web::post().to_async(handler::create_posts))
    );

    cfg.service(
        web::resource("/{uuid}")
            .route(web::get().to_async(handler::show))
            .route(web::delete().to_async(handler::destroy))
            .route(web::patch().to_async(handler::update))
    );
}
