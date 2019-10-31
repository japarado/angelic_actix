extern crate diesel;
extern crate dotenv;

use colored::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use std::env;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", &database_url[..].red()))
}

pub fn create_manager() -> PostgresConnectionManager {
    PostgresConnectionManager::new(format!("host={}, user={}", "localhost", "pam"), TlsMode::None).unwrap()
}
