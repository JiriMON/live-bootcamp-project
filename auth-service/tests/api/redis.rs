use auth_service::domain::{Email, LoginAttemptId, TwoFACode};

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

#[tokio::test]
async fn redis_store_and_return_2fa() {
    let mut app = TestApp::new().await;
    let email = Email::parse("a@b.cz".to_string()).unwrap();
    let login_attempt_id = LoginAttemptId::parse("ecd1ae84-a1ec-442e-befb-adb57a953d7e".to_string()).unwrap();
    let two_fa_code = TwoFACode::parse("987767".to_string()).unwrap();
    {
        let mut  two_fa_store = app.two_fa_code_store.write().await;
        
        let _ = two_fa_store.add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone()).await;
        match two_fa_store.get_code(&email).await {
            Ok((login_attempt_id2, code2)) => assert_eq!((login_attempt_id2.to_owned(),code2),(login_attempt_id,two_fa_code)),
            Err(_) => return
        }
        
        
    }
    app.clean_up().await;
}