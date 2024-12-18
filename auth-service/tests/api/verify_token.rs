use crate::helpers::{TestApp,get_random_email};
use auth_service::utils::constants::JWT_COOKIE_NAME;

 #[tokio::test]
 async fn should_return_200_valid_token() {
    let mut app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());

    let token = auth_cookie.value();

    let verify_token_body = serde_json::json!({
        "token": &token,
    });
    let response = app.post_verify_token(&verify_token_body).await;
    assert_eq!(response.status().as_u16(), 200);

    app.clean_up().await;
 }
 
 #[tokio::test]
 async fn should_return_401_if_invalid_token() {
    let mut app = TestApp::new().await;


    let test_cases = [
       serde_json::json!({
            "token": "abc",
        }),
    ];
    for test_case in test_cases.iter() {
        let response = app.post_verify_token(test_case).await; 
        assert_eq!(
            response.status().as_u16(),
            401,
            "Failed for input: {:?}",
            test_case
        );
    }
    app.clean_up().await;
 }

 #[tokio::test]
async fn should_return_401_if_banned_token() {
    let mut app = TestApp::new().await;
    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());

    let token = auth_cookie.value();

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 200);


    let verify_token_body = serde_json::json!({
        "token": &token,
    });
    let response = app.post_verify_token(&verify_token_body).await;
    assert_eq!(response.status().as_u16(), 401);

    app.clean_up().await;
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let mut app = TestApp::new().await;


    let test_cases = [
       serde_json::json!({
            "": "falses",
        }),
    ];
    for test_case in test_cases.iter() {
        let response = app.post_verify_token(test_case).await; // call `post_login`
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
    app.clean_up().await;
}