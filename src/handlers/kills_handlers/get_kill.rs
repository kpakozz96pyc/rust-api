use axum::extract::State;
use axum::Json;
use uuid::Uuid;

use crate::domain::models::kill::{KillError, Kill};
use crate::handlers::kills_handlers::KillResponse;
use crate::infra::errors::InfraError;
use crate::infra::repositories::kill_repository;
use crate::utils::PathExtractor;
use crate::AppState;

pub async fn get_kill(
    State(state): State<AppState>,
    PathExtractor(post_id): PathExtractor<Uuid>,
) -> Result<Json<KillResponse>, KillError> {
    let kill =
        kill_repository::get(&state.pool, post_id)
            .await
            .map_err(|db_error| match db_error {
                InfraError::InternalServerError => KillError::InternalServerError,
                InfraError::NotFound => KillError::NotFound(post_id),
            })?;

    Ok(Json(adapt_kills_to_kills_response(kill)))
}

fn adapt_kills_to_kills_response(kill: Kill) -> KillResponse {
    KillResponse {
        id: kill.id,
        killer: kill.killer,
        killed: kill.killed,
        kill_date: kill.kill_date,
        range: kill.range,
        server: kill.server
    }
}
