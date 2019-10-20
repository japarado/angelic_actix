use crate::schema::posts;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug, Queryable, Eq, PartialEq)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String
}