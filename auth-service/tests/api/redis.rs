use crate::helpers::TestApp;

#[tokio::test]
async fn redis_returns_stored_token() {
    let mut app = TestApp::new().await;
    let token = "abc".to_string();

    {
        let mut  banned_token_store = app.banned_token_store.write().await;
        
        let _ = banned_token_store.add_token_to_banned_store(token.clone()).await;
        let contains_token = banned_token_store
            .verify_token_in_banned_store(token.as_str())
            .await
            .expect("Failed to check if token is banned");
        assert!(contains_token);
        
    }
    app.clean_up().await;
}