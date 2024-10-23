use crate::{
    app_state::AppState,
    domain::AuthAPIError,
    utils::auth::validate_token,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use serde::{Deserialize, Serialize};




pub async fn verify_token(
    // Use Axum's state extractor to pass in AppState
    State(_state): State<AppState>,
    Json(request): Json<VerifyRequest>,
) ->Result<impl IntoResponse, AuthAPIError> {
 
    if request.token.is_empty(){
        return Err(AuthAPIError::InvalidCredentials)
    }

    if validate_token(&request.token).await.is_err() {
        return Err(AuthAPIError::InvalidToken)
    }

    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub struct VerifyRequest {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct VerifyResponse {
    pub message: String,
}