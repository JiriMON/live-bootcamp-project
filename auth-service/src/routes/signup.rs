use axum::{http::StatusCode, response::IntoResponse};

async fn signup() -> impl IntoResponse {
    StatusCode::OK.into_response()
}