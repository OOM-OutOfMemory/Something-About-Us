use url::Url;

use crate::domain::oauth::{
    auth_session::AuthSession, error::SAUOAuthDomainError, sau_jwt::OAuthAccessToken,
};

#[async_trait::async_trait]
pub trait OAuthRequest: Send + Sync {
    async fn login(&self) -> Result<(Url, AuthSession), SAUOAuthDomainError>;
    async fn callback(
        &self,
        code: String,
        pkce_verifier: String,
    ) -> Result<OAuthAccessToken, SAUOAuthDomainError>;
    async fn get_user_id(
        &self,
        access_token: OAuthAccessToken,
    ) -> Result<String, SAUOAuthDomainError>;
}
