use axum::{http::StatusCode, response::IntoResponse};


async fn verify_token() -> impl IntoResponse {
    StatusCode::OK.into_response()
}