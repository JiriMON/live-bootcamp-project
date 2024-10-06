use axum::{http::StatusCode, response::IntoResponse};

async fn login() -> impl IntoResponse {
    StatusCode::OK.into_response()
}