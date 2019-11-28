use crate::models::Post;
use serde::Serialize;

#[derive(Serialize)]
pub struct Multiple {
    pub posts: Vec<Post>
}

#[derive(Serialize)]
pub struct Single {
    pub post: Post
}

#[derive(Serialize)]
pub struct ErrorResponse {
    message: String
}
