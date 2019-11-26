extern crate diesel;
extern crate dotenv;

use colored::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;
use std::vec::Vec;

use std::io;
use futures::Future;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use r2d2_postgres::TlsMode;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", &database_url[..].red()))
}

pub fn create_pool() {
    let manager = PostgresConnectionManager::new(String::from("postgres://pam:pam@localhost/angelic_actix"), TlsMode::None);

    let mut pool: Pool<PostgresConnectionManager>;
    match manager {
        Ok(verified_manager) => pool = Pool::new(verified_manager).unwrap(),
        Err(e) => println!("Error establishing pool")
    };
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
            Ok(existing_user) => {}
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
