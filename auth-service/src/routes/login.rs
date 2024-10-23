use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password, User},
    utils::auth::generate_auth_cookie,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use axum_extra::extract::CookieJar;

pub async fn login(
    // Use Axum's state extractor to pass in AppState
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    // Create a new `User` instance using data in the `request`
    let email = match Email::parse(request.email){
        Ok(email) => email,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)) 
    };
    let password = match Password::parse(request.password){
        Ok(password) => password,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials))
    }; 

    let user_store = state.user_store.read().await;

    // early return AuthAPIError::UserAlreadyExists if email exists in user_store.
    if user_store.validate_user(&email,&password).await.is_err() {
        return (jar,Err(AuthAPIError::IncorrectCredentials));
    }
 
    let user: User = match user_store.get_user(&email).await{
        Ok(user) => user,
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
    };


    let auth_cookie = match generate_auth_cookie(&user.email) {
        Ok(cookie) => cookie,
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
    };

    let updated_jar = jar.add(auth_cookie);

    (updated_jar, Ok(StatusCode::OK.into_response()))
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
