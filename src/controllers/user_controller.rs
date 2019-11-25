use crate::database::establish_connection;
use crate::models::{NewPost, NewUser, Post, User};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[get("")]
pub fn index() -> impl Responder {
    use crate::schema::users::dsl::*;

    let connection = establish_connection();
    let result = users.order(id.asc()).load::<User>(&connection);

    HttpResponse::Ok().json(result.unwrap())
}

#[post("")]
pub fn store(form: web::Form<NewUser>) -> impl Responder {
    use crate::schema::users::dsl::*;

    let new_user = NewUser {
        email: form.email.to_string(),
        hash: form.hash.to_string(),
    };
    println!("{:?}", new_user);

    let connection = establish_connection();

    let user = diesel::insert_into(users)
        .values(new_user)
        .load::<User>(&connection)
        .expect("Can't create new user");

    HttpResponse::Ok().json(user)
}
