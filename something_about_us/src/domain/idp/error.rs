use thiserror::Error;

#[derive(Debug, Error)]
pub enum SupportIdpError {
    #[error("Invalid value: {0}")]
    CastingError(String),
}
