extern crate diesel;
extern crate dotenv;

use colored::*;
use diesel::mysql::MysqlConnection;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;

// pub fn establish_connection() -> MysqlConnection {
//     let database_url: str = env::var("DATABASE_URL")
//         .expect("DATABASE_URL must be set");
//     MysqlConnection::establish(&database_url)
//         .expect(&format!("Error connection to {}", &database_url[..].red()))
// }

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connection to {}", &database_url[..].red()))
}
