use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::sync::{Arc, Mutex};

use crate::domain::transacao_model::Transacao;
use crate::port::transacao_repository::{RepositoryError, TransacaoRepository};

pub struct VecRepository {
    memory: Arc<Mutex<Vec<Transacao>>>,
}

impl VecRepository {
    pub fn new() -> Self {
        Self {
            memory: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl Default for VecRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TransacaoRepository for VecRepository {
    async fn insert_transacao(&self, transacao: Transacao) -> Result<(), RepositoryError> {
        let vec_clone = Arc::clone(&self.memory);

        let handle = tokio::spawn(async move {
            // TODO: remove unwrap
            vec_clone.lock().unwrap().push(transacao);
        });

        // TODO: remove unwrap
        handle.await.unwrap();
        Ok(())
    }

    async fn delete_transacao(&self, id: usize) -> Result<(), RepositoryError> {
        let vec_clone = Arc::clone(&self.memory);

        let handle = tokio::spawn(async move {
            // TODO: remove unwrap
            let mut v = vec_clone.lock().unwrap();

            for i in 0..v.len() {
                // TODO: remove unwrap
                if v.get(i).unwrap().id == id {
                    v.remove(i);
                    break;
                }
            }
        });

        // TODO: remove unwrap
        handle.await.unwrap();
        Ok(())
    }

    async fn get_all_transacoes(
        &self,
        min: DateTime<Utc>,
        max: DateTime<Utc>,
    ) -> Result<Vec<Transacao>, RepositoryError> {
        let vec_clone = Arc::clone(&self.memory);
        let mut result: Vec<Transacao> = Vec::new();

        let handle = tokio::spawn(async move {
            // TODO: remove unwrap
            let v = vec_clone.lock().unwrap();

            for i in 0..v.len() {
                // TODO: remove unwrap
                let element = v.get(i).unwrap();
                if element.date_time >= min && element.date_time <= max {
                    result.push(element.clone());
                }
            }
            result
        });

        // TODO: remove unwrap
        let result = handle.await.unwrap();
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::transacao_model::Transacao;
    use chrono::{TimeZone, Utc};

    #[tokio::test]
    async fn insert_delete() {
        let v = VecRepository::new();

        let t1 = Transacao::new(1, 10.2, Utc::now());
        let t2 = Transacao::new(2, 20.2, Utc::now());
        let t3 = Transacao::new(3, 20.2, Utc::now());

        v.insert_transacao(t1.clone()).await.unwrap();
        v.insert_transacao(t2.clone()).await.unwrap();
        v.insert_transacao(t3.clone()).await.unwrap();

        assert_eq!(
            v.memory.lock().as_deref().unwrap(),
            &vec![t1.clone(), t2.clone(), t3.clone()]
        );

        v.delete_transacao(1).await.unwrap();

        assert_eq!(
            v.memory.lock().as_deref().unwrap(),
            &vec![t2.clone(), t3.clone()]
        );
    }

    #[tokio::test]
    async fn list_all() {
        let v = VecRepository::new();

        let t1 = Transacao::new(
            1,
            10.2,
            chrono::Local
                .with_ymd_and_hms(2026, 10, 24, 21, 54, 0)
                .unwrap()
                .to_utc(),
        );
        let t2 = Transacao::new(
            2,
            20.2,
            chrono::Local
                .with_ymd_and_hms(2026, 10, 22, 21, 54, 0)
                .unwrap()
                .to_utc(),
        );
        let t3 = Transacao::new(
            3,
            30.2,
            chrono::Local
                .with_ymd_and_hms(2026, 10, 21, 21, 54, 0)
                .unwrap()
                .to_utc(),
        );

        v.insert_transacao(t1.clone()).await.unwrap();
        v.insert_transacao(t2.clone()).await.unwrap();
        v.insert_transacao(t3.clone()).await.unwrap();

        let max1 = chrono::Local
            .with_ymd_and_hms(2026, 10, 25, 0, 0, 0)
            .unwrap()
            .to_utc();
        let min1 = chrono::Local
            .with_ymd_and_hms(2026, 10, 21, 0, 0, 0)
            .unwrap()
            .to_utc();

        assert_eq!(
            v.get_all_transacoes(min1, max1).await.unwrap(),
            vec![t1.clone(), t2.clone(), t3.clone()]
        );

        let max2 = chrono::Local
            .with_ymd_and_hms(2026, 10, 25, 0, 0, 0)
            .unwrap()
            .to_utc();
        let min2 = chrono::Local
            .with_ymd_and_hms(2026, 10, 24, 0, 0, 0)
            .unwrap()
            .to_utc();

        assert_eq!(
            v.get_all_transacoes(min2, max2).await.unwrap(),
            vec![t1.clone()]
        );
    }
}
