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
mod responders;
mod routes;
mod schema;
mod seeders;

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

    // database::seed();

    let pool = database::create_pool();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
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
            .service(admin)
    })
    .bind(address)
    .unwrap()
    .run()
    .unwrap();
}

#[get("/")]
fn index() -> impl Responder {
    HttpResponse::Ok().body("Root")
}

#[get("/admin")]
fn admin() -> impl Responder {
    HttpResponse::Ok().json(responders::GenericResponse {
        message: String::from("Admin Console"),
    })
}
