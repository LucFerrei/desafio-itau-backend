use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Transacao {
    pub id: usize,
    value: f64,
    pub date_time: DateTime<Utc>,
}

impl Transacao {
    pub fn new(id: usize, value: f64, date_time: DateTime<Utc>) -> Self {
        Self {
            id,
            value,
            date_time: date_time.to_utc(),
        }
    }
}
