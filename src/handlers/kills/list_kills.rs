use axum::extract::{Query, State};
use axum::Json;

use crate::domain::models::kill::{PostError, Kill};
use crate::handlers::kills::{ListKillsResponse, KillResponse};
use crate::infra::repositories::kill_repository::{get_all, PostsFilter};
use crate::AppState;

pub async fn list_posts(
    State(state): State<AppState>,
    Query(params): Query<PostsFilter>,
) -> Result<Json<ListKillsResponse>, PostError> {
    let posts = get_all(&state.pool, params)
        .await
        .map_err(|_| PostError::InternalServerError)?;

    Ok(Json(adapt_posts_to_list_posts_response(posts)))
}

fn adapt_post_to_post_response(post: Kill) -> KillResponse {
    KillResponse {
        id: post.id,
        title: post.title,
        body: post.body,
        published: post.published,
    }
}

fn adapt_posts_to_list_posts_response(posts: Vec<Kill>) -> ListKillsResponse {
    let posts_response: Vec<KillResponse> =
        posts.into_iter().map(adapt_post_to_post_response).collect();

    ListKillsResponse {
        posts: posts_response,
    }
}
