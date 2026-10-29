use crate::domain::estatistica_model::Estatistica;
use crate::{app::AppState, errors::AppError};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};

pub fn estatistica_route(app_state: AppState) -> Router {
    Router::new()
        .route("/estatistica/{time_diff}", get(estatistica))
        .route("/estatistica", get(estatistica))
        .with_state(app_state)
}

async fn estatistica(
    State(state): State<AppState>,
    diff: Option<Path<i64>>,
) -> Result<Json<Estatistica>, AppError> {
    let estatistica = match diff {
        Some(Path(time_diff)) => state
            .transacao_service
            .get_estatistica(time_diff)
            .await
            .unwrap(),
        None => state.transacao_service.get_estatistica(60).await.unwrap(),
    };

    Ok(Json(estatistica))
}
