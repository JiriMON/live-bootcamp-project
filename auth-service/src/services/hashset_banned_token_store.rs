use std::collections::HashSet;
use crate::domain::{BannedTokenStore,BannedTokenStoreError};

#[derive(Default)]
pub struct HashsetBannedTokenStore {
    tokens: HashSet<String>
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn add_token_to_banned_store(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
      
        self.tokens.insert(token);
        Ok(())
    }

    async fn verify_token_in_banned_store(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        Ok(self.tokens.contains(token))
    }
}

#[cfg(test)]
    mod tests {
        use super::*;
    
        #[tokio::test]
        async fn test_token_to_banned_store() {
            let mut banned_token_store = HashsetBannedTokenStore::default();
            let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhQGJjLmN6IiwiZXhwIjoxNzI5NzU0MjI2fQ.o6Tm7n8P2INdNKmVqFdefbVOw6Gb9S4aHBvF_buQZ_M".to_owned();
    
           
            let result = banned_token_store.add_token_to_banned_store(token.clone()).await;
            assert!(result.is_ok());
    
           
            assert!(banned_token_store.tokens.contains(&token));
        }

        #[tokio::test]
        async fn test_verify_token_in_banned_store() {
            let mut banned_token_store = HashsetBannedTokenStore::default();
            let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhQGJjLmN6IiwiZXhwIjoxNzI5NzU0MjI2fQ.o6Tm7n8P2INdNKmVqFdefbVOw6Gb9S4aHBvF_buQZ_M".to_string();
    
   
            banned_token_store.add_token_to_banned_store(token.clone()).await;
            
    
     
            let result = banned_token_store.verify_token_in_banned_store(&token).await;
            assert!(result.is_ok());
        }
    }