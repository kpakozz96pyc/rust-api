use axum::Json;
use crate::domain::models::kill::{KillError};
use crate::discord::discord::collect_messages;

pub async fn load_kills_from_discord(
) -> Result<Json<&'static str>, KillError> {
    collect_messages()
        .await
        .map_err(|_| KillError::InternalServerError)?;

    Ok(Json("Ok"))
}
