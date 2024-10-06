use axum::{http::StatusCode, response::IntoResponse};

async fn logout() -> impl IntoResponse {
    StatusCode::OK.into_response()
}