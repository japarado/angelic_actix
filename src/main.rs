// Establish database actors
#[macro_use]
extern crate diesel;
extern crate actix;
extern crate dotenv;

mod database;
mod models;
mod schema;

use actix::prelude::*;
use actix::sync;
use actix_web::Error;
use actix_web::{delete, get, patch, post};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use diesel::mysql::MysqlConnection;
use diesel::pg::PgConnection;
use diesel::query_dsl::*;
use dotenv::dotenv;
use models::post::{NewPost, Post};
use std::sync::Mutex;

pub struct DbExecutor(PgConnection);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl Message for NewPost {
    type Result = Result<Post, Error>;
}

impl Handler<NewPost> for DbExecutor {
    type Result = Result<Post, Error>;

    fn handle(&mut self, msg: CreatePost, _: &mut Self::Context) -> Self::Result {
        use crate::schema::posts::dsl::*;

        // Create insertion model
        let new_post = NewPost {
            title: msg.title.clone(),
            body: msg.body.clone(),
        };

        // normal diesel operation
        let results = diesel::insert_into(posts)
            .values(&new_post)
            .get_result(&self.0)
            .expect("Error loading post");

        Ok(results)
    }
}

struct State {
    db: Mutex<Addr<DbExecutor>>,
}

fn main() {
    dotenv().ok();

    let sys = actix::System::new("anglic-actix");

    // Start 3 parallel DB executors
    let addr = sync::SyncArbiter::start(3, || DbExecutor(database::establish_connection()));

    let app_state = web::Data::new(State {
        db: Mutex::new(addr.clone()),
    });

    HttpServer::new(move || App::new().register_data(app_state.clone()).service(index))
        .bind("localhost:8000")
        .unwrap()
        .run()
        .unwrap();
}

/**
 * Root endpoint of the API
 **/
#[post("/")]
// /// Async handler
fn index(
    (form, state, req): (web::Form<NewPost>, web::Data<State>, HttpRequest),
) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let new_post = NewPost {
        title: form.title.to_string(),
        body: form.body.to_string(),
    };

    let db = state.db.lock().unwrap();

    db.send(NewPost)
        .from_err()
        .and_then(|res| match res {
            Ok(post) => Ok(HttpResponse::Ok().json(post)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}
