pub mod post_responders;
pub mod user_responders;

use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub message: String,
}
