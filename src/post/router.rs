use actix_web::{web};
use crate::post::handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(handler::get_posts))
    );

    cfg.service(
        web::resource("/posts")
            .route(web::get().to_async(handler::index))
    );
}
