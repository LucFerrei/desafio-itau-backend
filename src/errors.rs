use crate::routes::transacao::TransacaoError;
use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Internal Error")]
    InternalError,

    #[error("TransacaoError: {0}")]
    TransacaoError(#[from] TransacaoError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::InternalError => (
                StatusCode::BAD_REQUEST,
                "Has something wrong with your request".to_string(),
            ),

            AppError::TransacaoError(e) => (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()),
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}
