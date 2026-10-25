use crate::{
    adapter::vec_memory::VecRepository, app::AppState,
    domain::services::transacao::TransacaoService,
};
use std::sync::Arc;

pub mod adapter;
mod app;
pub mod domain;
pub mod errors;
pub mod port;
pub mod routes;

#[tokio::main]
async fn main() {
    let repo = VecRepository::default();

    let transacao_service = TransacaoService::new(Arc::new(repo));

    let app_state = AppState {
        transacao_service: Arc::new(transacao_service),
    };

    let app = app::create_app(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed bind listener");

    axum::serve(listener, app)
        .await
        .expect("Failed to start serve")
}
