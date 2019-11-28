extern crate diesel;
extern crate dotenv;

use colored::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;
use std::vec::Vec;

use futures::Future;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use r2d2_postgres::TlsMode;
use std::io;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", &database_url[..].red()))
}

pub fn create_pool() -> Pool<PostgresConnectionManager> {
    let manager = PostgresConnectionManager::new(
        String::from("postgres://pam:pam@localhost/angelic_actix"),
        TlsMode::None,
    )
    .expect("Cannot create maanger");
    Pool::new(manager).expect("Cannot create pool")

    // let mut pool: Pool<PostgresConnectionManager>;
    // match manager {
    //     Ok(verified_manager) => pool = Pool::new(verified_manager).unwrap(),
    //     Err(e) => println!("Error establishing pool")
    // };
}

pub fn seed() {
    delete();
    user_seeder();
    post_seeder();
}

fn user_seeder() {
    use crate::models::{NewUser, User};
    use crate::schema::users::dsl::*;

    let connection = establish_connection();

    let mut user_insert_array: Vec<NewUser> = Vec::new();
    for counter in 1..5 {
        let new_user = NewUser {
            email: format!("user{}@mail.com", counter),
            hash: String::from("password"),
        };

        println!("Seeding {:?}", new_user);

        let user_exists = users
            .filter(email.eq(format!("{}", &new_user.email)))
            .first::<User>(&connection);
        match user_exists {
            Ok(_existing_user) => {}
            Err(_e) => {
                diesel::insert_into(users)
                    .values(new_user)
                    .execute(&connection)
                    .expect("Can't create new user");
            }
        }

        // user_insert_array.push(new_user);
    }
}

fn post_seeder() {
    use crate::models::{NewPost, Post, User};
    use crate::schema::posts::dsl::*;
    use crate::schema::users::dsl::*;

    let posts_per_user = 10;

    let connection = establish_connection();
    let retrieved_users = users.load::<User>(&connection).unwrap();

    for user in retrieved_users.iter() {
        for post_count in 1..posts_per_user + 1 {
            let new_post = NewPost {
                title: format!("Post Number {} by {}", post_count, user.id),
                body: format!(
                    "Hi guys, this is {} and this is post number {}",
                    user.email, post_count
                ),
                user_id: user.id,
            };
            println!("Seeding {:?}", new_post);

            diesel::insert_into(posts)
                .values(new_post)
                .execute(&connection)
                .expect("Can't create new post");
        }
    }
}

fn delete() {
    use crate::schema::users::dsl::*;
    use crate::schema::posts::dsl::*;
    let connection = establish_connection();

    diesel::delete(posts).execute(&connection).expect("Can't delete posts table");
    diesel::delete(users).execute(&connection).expect("Can't delete users table");
}

