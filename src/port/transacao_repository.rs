use crate::domain::transacao_model::Transacao;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use thiserror::Error;

#[async_trait]
pub trait TransacaoRepository {
    async fn insert_transacao(&self, transacao: Transacao) -> Result<(), RepositoryError>;
    async fn delete_transacao(&self, id: usize) -> Result<(), RepositoryError>;
    async fn get_all_transacoes(
        &self,
        min: DateTime<Utc>,
        max: DateTime<Utc>,
    ) -> Result<Vec<Transacao>, RepositoryError>;
}

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Transacao not found")]
    TransacaoNotFound,
}
