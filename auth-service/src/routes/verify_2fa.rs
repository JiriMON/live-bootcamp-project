use axum::{http::StatusCode, response::IntoResponse};

async fn verify_2fa() -> impl IntoResponse {
    StatusCode::OK.into_response()
}