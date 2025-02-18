use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use bcrypt::BcryptError;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum AppError {
    #[error("statuscode")]
    Status(StatusCode),
    #[error("bcrypt error")]
    Hash(#[from] BcryptError),
    #[error("Ulid Encode Error")]
    UEncode(#[from] ulid::EncodeError),
    #[error("Ulid Decode Error")]
    UDecode(#[from] ulid::DecodeError),
    #[error("failed to deserialize")]
    Json(#[from] serde_json::Error),
    #[error("pool failed to execute")]
    Pool(#[from] sqlx::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (body, code) = match self {
            AppError::Status(e) => ("", e),
            _ => ("Unknown", StatusCode::INTERNAL_SERVER_ERROR),
        };
        Response::builder().status(code).body(body.into()).unwrap()
    }
}
