use crate::routes::transacao::transacao_route;
use axum::Router;

pub fn create_app() -> Router {
    Router::new().merge(transacao_route())
}
