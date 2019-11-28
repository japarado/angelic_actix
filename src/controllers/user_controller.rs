use crate::database::establish_connection;
use crate::models::{NewPost, NewUser, Post, User};
use crate::responders::{user_responders, GenericResponse};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[get("")]
pub fn index() -> impl Responder {
    use crate::schema::users::dsl::*;

    let connection = establish_connection();
    let query = users.order(id.asc()).load::<User>(&connection);

    match query {
        Ok(all_users) => {
            let response = user_responders::Multiple { users: all_users };
            HttpResponse::Ok().json(response)
        }
        Err(err) => {
            let response = GenericResponse {
                message: format!("Error retrieving users: {}", err),
            };
            HttpResponse::Ok().json(response)
        }
    }
}

#[derive(Serialize, Deserialize)]
struct GetResponse {
    user: User,
    posts: Vec<Post>,
}

#[get("/{id}")]
pub fn get(path: web::Path<(i32)>) -> impl Responder {
    let request_user_id = path.to_string().parse::<i32>().unwrap();
    use crate::schema::users::dsl::*;

    let connection = establish_connection();
    let user_query = users.find(request_user_id).first::<User>(&connection);

    match user_query {
        Ok(user) => {
            let user_post_query = Post::belonging_to(&user).load::<Post>(&connection);
            match user_post_query {
                Ok(user_posts) => {
                    let get_response = GetResponse {
                        user: user,
                        posts: user_posts,
                    };
                    HttpResponse::Ok().json(get_response)
                }
                Err(_e) => HttpResponse::Ok().json("Did not find some user posts"),
            }
        }
        Err(_e) => HttpResponse::Ok().json("Cannot find user"),
    }
}

#[post("")]
pub fn store(form: web::Form<NewUser>) -> impl Responder {
    use crate::schema::users::dsl::*;

    let new_user = NewUser {
        email: form.email.to_string(),
        hash: form.hash.to_string(),
    };

    let connection = establish_connection();

    let user = diesel::insert_into(users)
        .values(new_user)
        .load::<User>(&connection)
        .expect("Can't create new user");

    HttpResponse::Ok().json(user)
}

#[patch("/{id}")]
pub fn update(path: web::Path<(i32)>, form: web::Form<NewUser>) -> impl Responder {
    use crate::schema::users::dsl::*;

    let new_user = NewUser {
        email: form.email.to_string(),
        hash: form.hash.to_string(),
    };

    let connection = establish_connection();
}

#[delete("/{id}")]
pub fn delete(path: web::Path<(i32)>) -> impl Responder {
    HttpResponse::Ok().json("User delete root")
}
