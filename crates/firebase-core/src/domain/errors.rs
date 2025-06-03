use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Invalid Firebase configuration: {0}")]
    InvalidConfig(String),

    #[error("Authentication failure: {0}")]
    AuthError(String),

    #[error("Network or HTTP error: {0}")]
    NetworkError(String),

    #[error("Unexpected error: {0}")]
    Unexpected(String),
}
