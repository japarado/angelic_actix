use crate::database;
use crate::models::{NewPost, Post};
use actix_web::{get, patch, post, web, HttpResponse, Responder};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[get("")]
fn index() -> impl Responder {
    use crate::schema::posts::dsl::*;
    let connection = database::establish_connection();

    let results = posts.load::<Post>(&connection);

    match results {
        Ok(retrieved_posts) => HttpResponse::Ok().json(retrieved_posts),
        Err(_e) => HttpResponse::Ok().body("No posts in database"),
    }
}

#[get("/{id}")]
pub fn get(path: web::Path<(i32)>) -> impl Responder {
    let post_id: i32 = path.to_string().parse::<i32>().unwrap();

    use crate::schema::posts::dsl::*;
    let connection = database::establish_connection();
    let result = posts.filter(id.eq(post_id)).first::<Post>(&connection);

    match result {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(_e) => HttpResponse::NotFound().json(format!("Post {} not found", post_id)),
    }
}

#[post("")]
pub fn store(form: web::Form<NewPost>) -> impl Responder {
    let new_post = NewPost {
        title: form.title.to_string(),
        body: form.body.to_string(),
    };

    use crate::schema::posts::dsl::*;

    let connection = database::establish_connection();

    let post: Post = diesel::insert_into(posts)
        .values(&new_post)
        .get_result(&connection)
        .expect("Can't create post");

    HttpResponse::Ok().json(post)
}

#[patch("/{id}")]
pub fn update(path: web::Path<(i32)>, form: web::Form<NewPost>) -> impl Responder {
    let post_id: i32 = path.to_string().parse::<i32>().unwrap();

    use crate::schema::posts::dsl::*;
    let connection = database::establish_connection();
    let result = posts.filter(id.eq(post_id)).first::<Post>(&connection);
}

#[derive(Serialize, Deserialize)]
pub struct Entry<T, V> {
    pub key: T,
    pub value: V,
}

#[derive(Serialize, Deserialize)]
pub struct Context<T, V> {
    pub items: Vec<Entry<T, V>>,
}