use std::collections::HashMap;

use crate::domain::{
    data_stores::{TwoFACodeStore, TwoFACodeStoreError},
    email::Email,
    login_attempt_id::LoginAttemptId,
    two_fa_code::TwoFACode
};

#[derive(Default)]
pub struct HashmapTwoFACodeStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

// implement TwoFACodeStore for HashmapTwoFACodeStore
#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        
        if self.codes.contains_key(&email) {
            return Err(TwoFACodeStoreError::UnexpectedError);
        }
        self.codes.insert(email,(login_attempt_id,code));
        Ok(())
    }

   

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        match self.codes.get(email) {
            Some(code) => Ok(code.clone()),
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        }
    }

   
    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        match self.codes.remove(&email) {
            Some(_) => Ok(()),
            None    => Err(TwoFACodeStoreError::UnexpectedError)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_add_code() {
        let mut two_fa_store = HashmapTwoFACodeStore::default();
        let code: (Email, LoginAttemptId, TwoFACode) = (Email::parse("test@xy.com".to_owned()).unwrap(),LoginAttemptId::default(),TwoFACode::default());


        let result = two_fa_store.add_code(code.0.clone(),code.1.clone(),code.2.clone()).await;
        assert!(result.is_ok());


        let result = two_fa_store.add_code(code.0,code.1,code.2).await;
        assert_eq!(result, Err(TwoFACodeStoreError::UnexpectedError));
    }

    #[tokio::test]
    async fn test_get_code() {
        let mut two_fa_store = HashmapTwoFACodeStore::default();
        let code: (Email, LoginAttemptId, TwoFACode) = (Email::parse("test@xy.com".to_owned()).unwrap(),LoginAttemptId::default(),TwoFACode::default());

        let result = two_fa_store.add_code(code.0.clone(),code.1.clone(),code.2.clone()).await;
        assert!(result.is_ok());

        let result = two_fa_store.get_code(&code.0).await;        
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_remove_code() {
        let mut two_fa_store = HashmapTwoFACodeStore::default();
        let code: (Email, LoginAttemptId, TwoFACode) = (Email::parse("test@xy.com".to_owned()).unwrap(),LoginAttemptId::default(),TwoFACode::default());


        let result = two_fa_store.add_code(code.0.clone(),code.1.clone(),code.2.clone()).await;
        assert!(result.is_ok());
       
        let result = two_fa_store.remove_code(&code.0).await;
        assert!(result.is_ok());

        let result = two_fa_store.get_code(&code.0).await;        
        assert_eq!(result, Err(TwoFACodeStoreError::LoginAttemptIdNotFound ));
    }
}