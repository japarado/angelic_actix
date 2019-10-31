#[macro_use]
extern crate diesel;
extern crate actix;
extern crate dotenv;
extern crate futures;
extern crate r2d2;
extern crate  postgres;

mod controllers;
mod database;
mod models;
mod routes;
mod schema;

use actix_web::{
    get, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use dotenv::dotenv;
use std::env;
use futures::Future;
use r2d2::Pool;

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    dotenv().ok();

    let address = format!(
        "{}:{}",
        env::var("HOST").unwrap_or(String::from("localhost")),
        env::var("PORT").unwrap_or(String::from("8000"))
    );

    // r2d2 pool 
    let manager = database::create_manager();
    let pool = Pool::new(manager).unwrap();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
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
