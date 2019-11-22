use crate::controllers::post_controller;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .service(post_controller::index)
            .service(post_controller::get)
            .service(post_controller::store)
            .service(post_controller::update),
    );
}
