use actix_web::{HttpRequest, Responder};

pub fn get_posts(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}
