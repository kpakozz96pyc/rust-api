use axum::extract::State;
use axum::Json;

use crate::domain::models::kill::PostError;
use crate::handlers::kills::{CreateKillRequest, KillResponse};
use crate::infra::repositories::kill_repository;
use crate::utils::JsonExtractor;
use crate::AppState;

pub async fn create_kill(
    State(state): State<AppState>,
    JsonExtractor(new_post): JsonExtractor<CreateKillRequest>,
) -> Result<Json<KillResponse>, PostError> {
    let new_post_db = kill_repository::NewPostDb {
        title: new_post.title,
        body: new_post.body,
        published: false,
    };

    let created_post = kill_repository::insert(&state.pool, new_post_db)
        .await
        .map_err(PostError::InfraError)?;

    let post_response = KillResponse {
        id: created_post.id,
        title: created_post.title,
        body: created_post.body,
        published: created_post.published,
    };

    Ok(Json(post_response))
}
