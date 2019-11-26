#[macro_use]
extern crate diesel;
extern crate actix;
extern crate dotenv;
extern crate futures;
extern crate postgres;
extern crate r2d2;

mod controllers;
mod database;
mod models;
mod routes;
mod schema;

use actix_cors::Cors;
use actix_web::{get, http, middleware, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

// r2d2 setup

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    dotenv().ok();

    let address = format!(
        "{}:{}",
        env::var("HOST").unwrap_or(String::from("localhost")),
        env::var("PORT").unwrap_or(String::from("8000"))
    );

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    // .allowed_origin("http://localhost:3001")
                    .allowed_methods(vec!["GET", "PUT", "POST", "PATCH", "DELETE"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .wrap(middleware::Logger::default())
            .configure(routes::posts::config)
            .configure(routes::users::config)
            .service(index)
    })
    .bind(address)
    .unwrap()
    .run()
    .unwrap();
}

#[get("/")]
fn index() -> impl Responder {
    HttpResponse::Ok().body("API Root")
}
