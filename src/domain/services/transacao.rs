use std::sync::{Arc, Mutex};

use chrono::{DateTime, FixedOffset, Utc};

use crate::{
    domain::{estatistica_model::Estatistica, transacao_model::Transacao},
    errors::AppError,
    port::transacao_repository::TransacaoRepository,
    routes::transacao::TransacaoError,
};

pub struct TransacaoService {
    next: Arc<Mutex<usize>>,
    repo: Arc<dyn TransacaoRepository + Send + Sync>,
}

impl TransacaoService {
    pub fn new(repo: Arc<dyn TransacaoRepository + Send + Sync>) -> Self {
        Self {
            repo,
            next: Arc::new(Mutex::new(0)),
        }
    }

    pub async fn insert_transacao(
        &self,
        value: f64,
        date_time: DateTime<FixedOffset>,
    ) -> Result<(), AppError> {
        if value < 0.0 {
            return Err(AppError::TransacaoError(TransacaoError::ValueError));
        }

        if date_time > chrono::Utc::now() {
            return Err(AppError::TransacaoError(TransacaoError::TimeError));
        }

        let next_clone = Arc::clone(&self.next);
        let id = tokio::spawn(async move {
            let mut v = next_clone.lock().unwrap();
            let result = *v;

            *v += 1;

            result
        });

        let transacao = Transacao::new(id.await.unwrap(), value, date_time.to_utc());

        self.repo.insert_transacao(transacao).await.unwrap();

        Ok(())
    }

    pub async fn get_estatistica(&self, time_diff: i64) -> Result<Estatistica, AppError> {
        let max = Utc::now();
        let min = max - chrono::TimeDelta::seconds(time_diff);

        // TODO: remove this unwrap
        // add repo error in app error
        let transacoes = self.repo.get_all_transacoes(min, max).await.unwrap();
        println!("{:#?}", &transacoes);

        let estatistica = Estatistica::from_transacoes(&transacoes);
        Ok(estatistica)
    }
}
