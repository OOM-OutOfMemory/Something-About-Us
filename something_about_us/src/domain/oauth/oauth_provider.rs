use url::Url;

use crate::domain::{
    idp::supported_idp::SupportIdp,
    oauth::{auth_session::AuthSession, error::SAUOAuthDomainError, sau_jwt::OAuthAccessToken},
};

pub fn get_oauth_redirect_url(provider: SupportIdp) -> Url {
    Url::parse(
        format!(
            "http://127.0.0.1:3000/api/v1/oauth/{}/callback",
            provider.as_str()
        )
        .as_str(),
    )
    .unwrap()
}

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
