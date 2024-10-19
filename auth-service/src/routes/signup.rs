use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use crate::{
    app_state::AppState,
    domain::{AuthAPIError,User}
};

/* pub async fn signup(Json(request): Json<SignupRequest>) -> impl IntoResponse {
    StatusCode::OK.into_response()
} */

pub async fn signup(
    // TODO: Use Axum's state extractor to pass in AppState
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> impl IntoResponse {
    // Create a new `User` instance using data in the `request`
    let email = request.email;
    let password = request.password;
    
        // TODO: early return AuthAPIError::InvalidCredentials if:
    // - email is empty or does not contain '@'
    // - password is less than 8 characters
    
    if password.len() < 8 || !email.contains("@") {
        return Err(AuthAPIError::InvalidCredentials)
    }
    
    let user = User::new(email, password, request.requires_2fa);



    let mut user_store = state.user_store.write().await;

    
    // early return AuthAPIError::UserAlreadyExists if email exists in user_store.
    if user_store.get_user(&user.email).await.is_ok() {
        return Err(AuthAPIError::UserAlreadyExists);
    }
    //  instead of using unwrap, early return AuthAPIError::UnexpectedError if add_user() fails.
    if user_store.add_user(user).await.is_err() {
        return Err(AuthAPIError::UnexpectedError);
    }

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Serialize,Deserialize,Debug,PartialEq)]
pub struct SignupResponse {
    pub message: String,
}