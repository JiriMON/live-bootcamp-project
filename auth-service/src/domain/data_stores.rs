use crate::domain::{Email,Password,TwoFACode,LoginAttemptId};
use super::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}


#[async_trait::async_trait]
pub trait UserStore {
    // Add the `add_user`, `get_user`, and `validate_user` methods.
    // Make sure all methods are async so we can use async user stores in the future
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: &Email, password: &Password) -> Result<(), UserStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum BannedTokenStoreError{
    TokenAlreadyBanned,
    UnexpectedError,
}

#[async_trait::async_trait]
pub trait BannedTokenStore {
    async fn add_token_to_banned_store(&mut self, token: String) -> Result<(), BannedTokenStoreError>;
    async fn verify_token_in_banned_store(&self, token: &str) -> Result<bool, BannedTokenStoreError>;
}



#[derive(Debug, PartialEq)]
pub enum TwoFACodeStoreError {
    LoginAttemptIdNotFound,
    UnexpectedError,
}
// This trait represents the interface all concrete 2FA code stores should implement
#[async_trait::async_trait]
pub trait TwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError>;
    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError>;
    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError>;
}





