use crate::helpers::{get_random_email, TestApp};
use auth_service::{
    domain::Email,
    routes::TwoFactorAuthResponse,
    utils::constants::JWT_COOKIE_NAME,

};


#[tokio::test]
async fn verify_2fa_returns_200() {
    let mut app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);

    let json_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");
    
    assert_eq!(json_body.message, "2FA required".to_owned());

    // assert that `json_body.login_attempt_id` is stored inside `app.two_fa_code_store`
 
    let code_tuple = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(random_email.clone()).unwrap())
        .await
        .unwrap();

    assert_eq!(code_tuple.0.as_ref(), json_body.login_attempt_id);
    
    let two_fa_code = code_tuple.1.as_ref();

    let verify_2fa_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": json_body.login_attempt_id,
        "2FACode": two_fa_code,
    });
    
    let response = app.post_verify_2fa(&verify_2fa_body).await;
    assert_eq!(response.status().as_u16(), 200);

    
    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());

    app.clean_up().await;
} 



#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let mut app = TestApp::new().await;
    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);

    let json_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(json_body.message, "2FA required".to_owned());

    // assert that `json_body.login_attempt_id` is stored inside `app.two_fa_code_store`

    let code_tuple = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(random_email.clone()).unwrap())
        .await
        .unwrap();

    assert_eq!(code_tuple.0.as_ref(), json_body.login_attempt_id);

    let test_cases = [ 
        serde_json::json!({
            "email": random_email,
            "loginAttemptId": "",
            "2FACode": "",
        }),
        serde_json::json!({
            "email": random_email,
            "loginAttemptId": code_tuple.0.as_ref().to_owned(),
            "2FACode": "",
        }),
        serde_json::json!({
            "email": random_email,
            "loginAttemptId": "",
            "2FACode": code_tuple.1.as_ref().to_owned(),
        }),
    ];
    for test_case in test_cases.iter(){
        let response = app.post_verify_2fa(&test_case).await;
        assert_eq!(response.status().as_u16(), 400);
    }

    app.clean_up().await;
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let mut app = TestApp::new().await;
    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);

    let json_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(json_body.message, "2FA required".to_owned());

    // assert that `json_body.login_attempt_id` is stored inside `app.two_fa_code_store`
 
    let code_tuple = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(random_email.clone()).unwrap())
        .await
        .unwrap();

    assert_eq!(code_tuple.0.as_ref(), json_body.login_attempt_id);

    let test_cases = [ 
        serde_json::json!({
            "email": "some@email.com",
            "loginAttemptId": "51a58c72-c3d1-4a8a-8ffc-ac69541a1629",
            "2FACode": "123456",
        }), 
        /* serde_json::json!({
            "email": random_email,
            "loginAttemptId": code_tuple.0.as_ref().to_owned(),
            "2FACode": "123456",
        }),
         serde_json::json!({
            "email": random_email,
            "loginAttemptId": "51a58c72-c3d1-4a8a-8ffc-ac69541a1629",
            "2FACode": "code_tuple.1.as_ref().to_owned()",
        }),  */
    ];
    for test_case in test_cases.iter(){
        let response = app.post_verify_2fa(&test_case).await;
        assert_eq!(response.status().as_u16(), 401);
    }

    app.clean_up().await;
}

#[tokio::test]
async fn should_return_401_if_old_code() {
    // Call login twice. Then, attempt to call verify-fa with the 2FA code from the first login requet. This should fail. 
    let mut app = TestApp::new().await;
    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);

    let json_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(json_body.message, "2FA required".to_owned());

    // assert that `json_body.login_attempt_id` is stored inside `app.two_fa_code_store`

    let code_tuple = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(random_email.clone()).unwrap())
        .await
        .unwrap();

    assert_eq!(code_tuple.0.as_ref(), json_body.login_attempt_id);

    let verify_2fa_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": code_tuple.0.as_ref().to_owned(),
        "2FACode": code_tuple.1.as_ref().to_owned(),
    });
    let response = app.post_verify_2fa(&verify_2fa_body).await;
    assert_eq!(response.status().as_u16(), 200);

    let response = app.post_verify_2fa(&verify_2fa_body).await;
    assert_eq!(response.status().as_u16(), 401);

    app.clean_up().await;
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let mut app = TestApp::new().await;
    let random_email = get_random_email(); // Call helper method to generate email 

    // add more malformed input test cases
    let test_cases = [
       serde_json::json!({
            "email": random_email,
            "loginAttemptId": true,
            "2FAcode": "test",
        }),
        serde_json::json!({
            "email": "",
            "loginAttemptId": "1234567789",
            "2FAcode": "123456",
        }),
        serde_json::json!({
            "email": "password123",
            "loginAttemptId": true,
        })
    ];

    for test_case in test_cases.iter() {
        let response = app.post_verify_2fa(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }

    app.clean_up().await;
}

