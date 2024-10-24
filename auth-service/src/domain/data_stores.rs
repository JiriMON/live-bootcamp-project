use crate::domain::{Email,Password};
use super::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}
#[derive(Debug, PartialEq)]
pub enum BannedTokenStoreError{
    TokenAlreadyBanned,
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

#[async_trait::async_trait]
pub trait BannedTokenStore {
    async fn add_token_to_banned_store(&mut self, token: String) -> Result<(), BannedTokenStoreError>;
    async fn verify_token_in_banned_store(&self, token: &str) -> Result<bool, BannedTokenStoreError>;
}