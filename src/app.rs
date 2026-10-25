use std::sync::Arc;

use crate::{domain::services::transacao::TransacaoService, routes::transacao::transacao_route};
use axum::Router;

#[derive(Clone)]
pub struct AppState {
    pub transacao_service: Arc<TransacaoService>,
}

pub fn create_app(app_state: AppState) -> Router {
    Router::new().merge(transacao_route(app_state))
}
