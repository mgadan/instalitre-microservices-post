use actix_web::{web, HttpResponse};
use crate::post::handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(handler::get_posts))
    );
}
