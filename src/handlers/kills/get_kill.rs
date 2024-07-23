use axum::extract::State;
use axum::Json;
use uuid::Uuid;

use crate::domain::models::kill::{PostError, Kill};
use crate::handlers::kills::KillResponse;
use crate::infra::errors::InfraError;
use crate::infra::repositories::kill_repository;
use crate::utils::PathExtractor;
use crate::AppState;

pub async fn get_kill(
    State(state): State<AppState>,
    PathExtractor(post_id): PathExtractor<Uuid>,
) -> Result<Json<KillResponse>, PostError> {
    let post =
        kill_repository::get(&state.pool, post_id)
            .await
            .map_err(|db_error| match db_error {
                InfraError::InternalServerError => PostError::InternalServerError,
                InfraError::NotFound => PostError::NotFound(post_id),
            })?;

    Ok(Json(adapt_post_to_post_response(post)))
}

fn adapt_post_to_post_response(post: Kill) -> KillResponse {
    KillResponse {
        id: post.id,
        title: post.title,
        body: post.body,
        published: post.published,
    }
}
