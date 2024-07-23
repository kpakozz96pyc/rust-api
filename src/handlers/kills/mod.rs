use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use create_kill::create_kill;
pub use get_kill::get_kill;
pub use list_kills::list_posts;

mod create_kill;
mod get_kill;
mod list_kills;

#[derive(Debug, Deserialize)]
pub struct CreateKillRequest {
    pub killer: String,
    pub killed: String,
    pub range: f32,
    pub gun: String,
    pub date: DateTime<Utc>,
    pub server: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KillResponse {
    id: Uuid,
    title: String,
    body: String,
    published: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListKillsResponse {
    posts: Vec<KillResponse>,
}
