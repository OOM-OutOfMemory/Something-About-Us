use uuid::Uuid;

use crate::{
    application::port::auth_session_repository::{AuthSessionCacheRepo, AuthSessionCacheRepoError},
    domain::oauth::auth_session::AuthSession,
    infrastructure::cache::memcached::repository::CacheRepoMchd,
};

#[async_trait::async_trait]
impl AuthSessionCacheRepo for CacheRepoMchd {
    async fn set_auth_session(
        &self,
        auth_session: &AuthSession,
    ) -> Result<(), AuthSessionCacheRepoError> {
        let mut client = self.get().await?;
        let body = sonic_rs::json!(auth_session);
        Ok(client
            .set(
                auth_session.id.to_string(),
                body.to_string(),
                Some(500),
                None,
            )
            .await
            .map_err(|e| AuthSessionCacheRepoError::SetAuthSessionError(e.to_string()))?)
    }

    async fn get_auth_session(
        &self,
        session_id: Uuid,
    ) -> Result<AuthSession, AuthSessionCacheRepoError> {
        let mut client = self.get().await?;
        let result = client
            .get(session_id.to_string())
            .await
            .map_err(|e| AuthSessionCacheRepoError::CacheConnectionError(e.to_string()))?;
        match result {
            Some(value) => {
                let auth_session: AuthSession = sonic_rs::from_slice(&value.data)
                    .map_err(|e| AuthSessionCacheRepoError::InvalidSessionId(e.to_string()))?;
                Ok(auth_session)
            }
            None => Err(AuthSessionCacheRepoError::SessionNotFound(
                session_id.to_string(),
            )),
        }
    }
}
