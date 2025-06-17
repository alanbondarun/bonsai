use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error")]
    DatabaseError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = StatusCode::INTERNAL_SERVER_ERROR;
        let body = "Internal server error";
        (status, body).into_response()
    }
}
