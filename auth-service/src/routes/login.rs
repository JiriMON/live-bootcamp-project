use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password, User},
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub async fn login(
    // Use Axum's state extractor to pass in AppState
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> impl IntoResponse {
    // Create a new `User` instance using data in the `request`
    let email =
        Email::parse(request.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password =
        Password::parse(request.password.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;

 

    let user_store = state.user_store.read().await;

    // early return AuthAPIError::UserAlreadyExists if email exists in user_store.
    if user_store.validate_user(&email,&password).await.is_err() {
        return Err(AuthAPIError::IncorrectCredentials);
    }
 
    let _user: User = match user_store.get_user(&email).await{
        Ok(user) => user,
        Err(_) => return Err(AuthAPIError::IncorrectCredentials),
    };

    
    let response = Json(LoginResponse {
        message: "User logged successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LoginResponse {
    pub message: String,
}
