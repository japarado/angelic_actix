use crate::database;
use crate::models::{NewPost, Post};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// Custom Responders
use crate::responders::post_responders;


#[get("")]
fn index() -> impl Responder {
    use crate::schema::posts::dsl::*;
    let connection = database::establish_connection();

    let results = posts.order(id.asc()).load::<Post>(&connection);

    match results {
        Ok(retrieved_posts) =>{
            HttpResponse::Ok().json(post_responders::Multiple{ posts: retrieved_posts })
        },
        Err(_e) => HttpResponse::Ok().body("No posts in database"),
    }
}

#[get("/{id}")]
pub fn get(path: web::Path<(i32)>) -> impl Responder {
    let post_id: i32 = path.to_string().parse::<i32>().unwrap();

    use crate::schema::posts::dsl::*;
    let connection = database::establish_connection();
    let result = posts.find(post_id).first::<Post>(&connection);

    match result {
        Ok(post) => HttpResponse::Ok().json(post_responders::Single{ post: post }),
        Err(_e) => HttpResponse::NotFound().json(format!("Post {} not found", post_id)),
    }
}

#[post("")]
pub fn store(form: web::Form<NewPost>) -> impl Responder {
    let new_post = NewPost {
        title: form.title.to_string(),
        body: form.body.to_string(),
        user_id: form.user_id,
    };

    use crate::schema::posts::dsl::*;

    let connection = database::establish_connection();

    let post: Post = diesel::insert_into(posts)
        .values(&new_post)
        .get_result(&connection)
        .expect("Can't create post");

    HttpResponse::Ok().json(post_responders::Single { post: post })
}

#[patch("/{id}")]
pub fn update(path: web::Path<(i32)>, form: web::Form<NewPost>) -> impl Responder {
    let post_id: i32 = path.to_string().parse::<i32>().unwrap();

    use crate::schema::posts::dsl::*;
    let connection = database::establish_connection();
    let result = posts.filter(id.eq(post_id)).first::<Post>(&connection);

    match result {
        Ok(post) => {
            let post_title = form.title.to_string();
            let post_body = form.body.to_string();
            let updated_post = diesel::update(&post)
                .set((title.eq(post_title), body.eq(post_body)))
                .get_result::<Post>(&connection)
                .unwrap();
            HttpResponse::Ok().json(updated_post)
        }
        Err(_e) => HttpResponse::Ok().body(format!("Post with ID of {} not found", post_id)),
    }
}

#[delete("/{id}")]
pub fn delete(path: web::Path<(i32)>) -> impl Responder {
    let post_id: i32 = path.to_string().parse::<i32>().unwrap();

    use crate::schema::posts::dsl::*;
    let connection = database::establish_connection();

    let deleted = diesel::delete(posts.find(post_id)).load::<Post>(&connection);

    match deleted {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(e) => HttpResponse::Ok().body(format!("{}", e)),
    }
}

