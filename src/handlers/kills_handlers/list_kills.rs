use axum::extract::{Query, State};
use axum::Json;

use crate::domain::models::kill::{KillError, Kill};
use crate::handlers::kills_handlers::{ListKillsResponse, KillResponse};
use crate::infra::repositories::kill_repository::{get_all, KillsFilter};
use crate::AppState;

pub async fn list_kills(
    State(state): State<AppState>,
    Query(params): Query<KillsFilter>,
) -> Result<Json<ListKillsResponse>, KillError> {
    let posts = get_all(&state.pool, params)
        .await
        .map_err(|_| KillError::InternalServerError)?;

    Ok(Json(adapt_kills_to_list_kills_response(posts)))
}

fn adapt_kill_to_kill_response(post: Kill) -> KillResponse {
    KillResponse {
        id: post.id,
        killer: post.killer,
        killed: post.killed
    }
}

fn adapt_kills_to_list_kills_response(posts: Vec<Kill>) -> ListKillsResponse {
    let posts_response: Vec<KillResponse> =
        posts.into_iter().map(adapt_kill_to_kill_response).collect();

    ListKillsResponse {
        kills: posts_response,
    }
}
