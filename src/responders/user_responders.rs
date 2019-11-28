use crate::models::{Post, User};
use serde::Serialize;

#[derive(Serialize)]
pub struct Multiple {
    pub users: Vec<User>,
}

#[derive(Serialize)]
pub struct Single {
    pub user: User,
}
