use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Serialize,Deserialize};
use axum_extra::extract::CookieJar;
use crate::{
    app_state::AppState, 
    domain::{AuthAPIError, Email, LoginAttemptId, TwoFACode}, 
    utils::auth::generate_auth_cookie,
};

pub async fn verify_2fa(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<Verify2FARequest>,
) -> Result<(CookieJar,impl IntoResponse), AuthAPIError> {
    let email = Email::parse(request.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let login_attempt_id = LoginAttemptId::parse(request.login_attempt_id.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let two_fa_code = TwoFACode::parse(request.two_fa_code.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;

    let mut two_fa_code_store = state.two_fa_code_store.write().await;

    // Call `two_fa_code_store.get_code`. If the call fails
    // return a `AuthAPIError::IncorrectCredentials`.
    let code_tuple = two_fa_code_store
      .get_code(&email)
      .await
      .map_err(|_| AuthAPIError::IncorrectCredentials)?;

    // Validate that the `login_attempt_id` and `two_fa_code`
    // in the request body matches values in the `code_tuple`. 
    // If not, return a `AuthAPIError::IncorrectCredentials`.
     if !code_tuple.0.eq(&login_attempt_id) || !code_tuple.1.eq(&two_fa_code) {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    let auth_cookie = match generate_auth_cookie(&email) {
        Ok(cookie) => cookie,
        Err(_) => return Err(AuthAPIError::UnexpectedError),
    };
    let updated_jar = jar.add(auth_cookie); 

    two_fa_code_store.remove_code(&email).await.map_err(|_| AuthAPIError::UnexpectedError)?;
         
    Ok((updated_jar,StatusCode::OK.into_response()))
}

// implement the Verify2FARequest struct. See the verify-2fa route contract in step 1 for the expected JSON body.
#[derive(Deserialize,Debug)]
pub struct Verify2FARequest {
    pub email: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
    #[serde(rename = "2FACode")]
    pub two_fa_code: String,
}

#[derive(Serialize,Deserialize,Debug,PartialEq)]
pub struct Verify2FAResponse {
    pub message: String,
}