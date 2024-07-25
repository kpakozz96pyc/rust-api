use axum::extract::State;
use axum::Json;

use crate::domain::models::kill::KillError;
use crate::handlers::kills_handlers::{CreateKillRequest, KillResponse};
use crate::infra::repositories::kill_repository;
use crate::utils::JsonExtractor;
use crate::AppState;

pub async fn create_kill(
    State(state): State<AppState>,
    JsonExtractor(new_post): JsonExtractor<CreateKillRequest>,
) -> Result<Json<KillResponse>, KillError> {
    let new_kill_db = kill_repository::NewKillDb {
        killer: new_post.killer,
        killed: new_post.killed,
        kill_date:new_post.kill_date
    };

    let created_post = kill_repository::insert(&state.pool, new_kill_db)
        .await
        .map_err(KillError::InfraError)?;

    let kill_response = KillResponse {
        id: created_post.id,
        killer: created_post.killer,
        killed: created_post.killed,
        kill_date: created_post.kill_date,
    };

    Ok(Json(kill_response))
}
