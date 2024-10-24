use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{
    app_state::AppState, domain::AuthAPIError, services::hashset_banned_token_store, utils::{auth::validate_token, constants::JWT_COOKIE_NAME}
};

pub async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    // Retrieve JWT cookie from the `CookieJar`
    // Return AuthAPIError::MissingToken is the cookie is not found
    let cookie = match jar.get(JWT_COOKIE_NAME) {
        Some(cookie)=> cookie,
        None => return(jar,Err(AuthAPIError::MissingToken))
    };

    let token = cookie.value().to_owned();

    // Validate JWT token by calling `validate_token` from the auth service.
    // If the token is valid you can ignore the returned claims for now.
    // Return AuthAPIError::InvalidToken is validation fails.

    if validate_token(&token, state.banned_token_store.clone()).await.is_err() {
        return(jar,Err(AuthAPIError::InvalidToken))
    }

    let mut banned_token_store = state.banned_token_store.write().await;
    
    if banned_token_store.add_token_to_banned_store(token).await.is_err() {
        return (jar,Err(AuthAPIError::UnexpectedError));
    }
    let jar = jar.remove(JWT_COOKIE_NAME);
    (jar, Ok(StatusCode::OK))
}