use crate::errors::AppError;
use axum::{Json, Router, routing::post};
use chrono::{DateTime, FixedOffset};
use serde::Deserialize;
use serde_json::{Value, json};
use thiserror::Error;

#[derive(Debug, Deserialize)]
struct TrasacaoPayload {
    value: f64,
    #[serde(alias = "dataHora")]
    date_time: DateTime<FixedOffset>,
}

pub fn transacao_route() -> Router {
    Router::new().route("/", post(transacao))
}

async fn transacao(Json(payload): Json<TrasacaoPayload>) -> Result<Json<Value>, AppError> {
    if payload.date_time > chrono::Utc::now() {
        return Err(AppError::TransacaoError(TransacaoError::TimeError));
    }

    if payload.value < 0.0 {
        return Err(AppError::TransacaoError(TransacaoError::ValueError));
    }

    Ok(Json(json!({
        "code": "201",
    })))
}

#[derive(Debug, Error)]
pub enum TransacaoError {
    #[error("Transaction cannot be in the future")]
    TimeError,

    #[error("Transaction cannot have negative value")]
    ValueError,
}
