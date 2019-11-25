use crate::controllers::user_controller;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(user_controller::index)
            .service(user_controller::store)
            .service(user_controller::get)
            .service(user_controller::delete)
            .service(user_controller::update),
    );
}
