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

use actix_web::{get, middleware, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    dotenv().ok();

    let address = format!(
        "{}:{}",
        env::var("HOST").unwrap_or(String::from("localhost")),
        env::var("PORT").unwrap_or(String::from("8000"))
    );

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(routes::posts::config)
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
