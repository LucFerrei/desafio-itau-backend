use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

use crate::domain::transacao_model::Transacao;

#[derive(Debug, Serialize)]
pub struct Estatistica {
    pub count: usize,
    pub sum: f64,
    pub avg: f64,
    pub min: f64,
    pub max: f64,
}

impl IntoResponse for Estatistica {
    fn into_response(self) -> axum::response::Response {
        let status = StatusCode::OK;
        let body: Json<Estatistica> = Json(self);

        (status, body).into_response()
    }
}

impl Estatistica {
    fn new(count: usize, sum: f64, avg: f64, min: f64, max: f64) -> Self {
        Self {
            count,
            sum,
            avg,
            min,
            max,
        }
    }

    pub fn from_transacoes(transacoes: &Vec<Transacao>) -> Self {
        if transacoes.is_empty() {
            return Self {
                count: 0,
                sum: 0.0,
                avg: 0.0,
                min: 0.0,
                max: 0.0,
            };
        }

        let count = transacoes.len();
        let (mut sum, mut min, mut max) = (0.0, 0.0, 0.0);

        for element in transacoes {
            let value = element.value;
            sum += value;

            if value > max {
                max = value
            }

            if value < min {
                min = value
            }
        }

        let avg = sum / (count as f64);

        Self::new(count, sum, avg, min, max)
    }
}
