use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug, Queryable, Eq, PartialEq, Identifiable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
}

pub struct NewUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

pub struct User {
    pub id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}
