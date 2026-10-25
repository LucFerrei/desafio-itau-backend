use crate::{app::AppState, errors::AppError};
use axum::{Json, Router, extract::State, routing::post};
use chrono::{DateTime, FixedOffset};
use serde::Deserialize;
use serde_json::{Value, json};
use thiserror::Error;

pub fn transacao_route(app_state: AppState) -> Router {
    Router::new()
        .route("/transacao", post(transacao))
        .with_state(app_state)
}

#[derive(Debug, Deserialize)]
pub struct TransacaoPayload {
    #[serde(alias = "valor")]
    pub value: f64,
    #[serde(alias = "dataHora")]
    pub date_time: DateTime<FixedOffset>,
}

// TODO:
async fn transacao(
    State(state): State<AppState>,
    Json(payload): Json<TransacaoPayload>,
) -> Result<Json<Value>, AppError> {
    state
        .transacao_service
        .insert_transacao(payload.value, payload.date_time)
        .await?;

    // FIX: fix this json. into response
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
    //TODO: add Json Error message
}
