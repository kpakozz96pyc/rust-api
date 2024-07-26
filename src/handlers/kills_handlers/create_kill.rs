use axum::extract::State;
use axum::Json;

use crate::domain::models::kill::KillError;
use crate::handlers::kills_handlers::{CreateKillRequest, KillResponse};
use crate::infra::repositories::kill_repository;
use crate::utils::JsonExtractor;
use crate::AppState;

pub async fn create_kill(
    State(state): State<AppState>,
    JsonExtractor(new_kill): JsonExtractor<CreateKillRequest>,
) -> Result<Json<KillResponse>, KillError> {
    let new_kill_db = kill_repository::NewKillDb {
        killer: new_kill.killer,
        killed: new_kill.killed,
        kill_date: new_kill.kill_date,
        range: new_kill.range,
        server: new_kill.server
    };

    let created_kill = kill_repository::insert(&state.pool, new_kill_db)
        .await
        .map_err(KillError::InfraError)?;

    let kill_response = KillResponse {
        id: created_kill.id,
        killer: created_kill.killer,
        killed: created_kill.killed,
        kill_date: created_kill.kill_date,
        range: created_kill.range,
        server: created_kill.server
    };

    Ok(Json(kill_response))
}
