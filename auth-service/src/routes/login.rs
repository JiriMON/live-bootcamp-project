use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use axum_extra::extract::CookieJar;
use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password, User, LoginAttemptId, TwoFACode},
    utils::auth::generate_auth_cookie,
};


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



  
    match user.requires_2fa {
        true => handle_2fa(&user.email, &state, jar).await,
        false => handle_no_2fa(&user.email, jar).await,
    }

    /* no longer necessary 
    let updated_jar = jar.add(auth_cookie);
   (updated_jar, Ok(StatusCode::OK.into_response()))
    */
   }


   
#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}


// The login route can return 2 possible success responses.
// This enum models each response!
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

// If a user requires 2FA, this JSON body should be returned!
#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}

async fn handle_2fa(
    email: &Email,
    state: &AppState, 
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    let login_attempt_id = LoginAttemptId::default();
    let two_fa_code = TwoFACode::default();
    // Return a TwoFactorAuthResponse. The message should be "2FA required".
    // The login attempt ID should be "123456". We will replace this hard-coded login attempt ID soon!
    if state.two_fa_code_store.write().await.add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone()).await.is_err(){
        return (jar,Err(AuthAPIError::UnexpectedError));
    }
    // send 2FA code via the email client. Return `AuthAPIError::UnexpectedError` if the operation fails.

    if state.email_client.write().await.send_email(email, "2FA code for your login", two_fa_code.as_ref()).await.is_err(){
        return (jar,Err(AuthAPIError::UnexpectedError));
    }

    let response = 
        Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse { 
            message: "Two factor authentification required".to_string(),
            login_attempt_id: login_attempt_id.as_ref().to_string()
        }));
    (jar, Ok((StatusCode::PARTIAL_CONTENT,response)))
}


async fn handle_no_2fa(
    email: &Email,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    let auth_cookie = match generate_auth_cookie(&email) {
        Ok(cookie) => cookie,
        Err(_) => return (jar,Err(AuthAPIError::UnexpectedError)),
    };
    let updated_jar = jar.add(auth_cookie);
    (updated_jar, Ok((StatusCode::OK,Json(LoginResponse::RegularAuth))))

}