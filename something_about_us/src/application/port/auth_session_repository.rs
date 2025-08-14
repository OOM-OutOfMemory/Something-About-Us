use uuid::Uuid;

use crate::domain::oauth::auth_session::AuthSession;

#[async_trait::async_trait]
pub trait AuthSessionCacheRepo: Send + Sync {
    async fn set_auth_session(
        &self,
        auth_session: &AuthSession,
    ) -> Result<(), AuthSessionCacheRepoError>;
    async fn get_auth_session(
        &self,
        session_id: Uuid,
    ) -> Result<AuthSession, AuthSessionCacheRepoError>;
}

#[derive(thiserror::Error, Debug)]
pub enum AuthSessionCacheRepoError {
    #[error("cache server connection error : {0}")]
    CacheConnectionError(String),

    #[error("failed to set auth session: {0}")]
    SetAuthSessionError(String),

    #[error("invalid session ID: {0}")]
    InvalidSessionId(String),

    #[error("session not found: {0}")]
    SessionNotFound(String),
}
