use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Internal Error")]
    InternalError,
}
