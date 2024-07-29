use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use create_kill::create_kill;
pub use get_kill::get_kill;
pub use list_kills::list_kills;
pub use load_kills_from_discord::load_kills_from_discord;

mod create_kill;
mod get_kill;
mod list_kills;

mod load_kills_from_discord;

#[derive(Debug, Deserialize)]
pub struct CreateKillRequest {
    pub killer: String,
    pub killed: String,
    pub kill_date: NaiveDateTime,
    pub range: f64,
    pub server: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KillResponse {
    id: Uuid,
    pub killer: String,
    pub killed: String,
    pub kill_date: NaiveDateTime,
    pub range: f64,
    pub server: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListKillsResponse {
    kills: Vec<KillResponse>,
}
