use std::{collections::HashMap, sync::Arc};
use url::Url;

use crate::{
    domain::{
        idp::supported_idp::SupportIdp,
        oauth::{
            auth_session::AuthSession, error::SAUOAuthDomainError, oauth_provider::OAuthRequest,
            sau_jwt::OAuthAccessToken,
        },
    },
    infrastructure::{config::types::Config, provider::github::GithubOAuthClient},
};

#[derive(Clone)]
pub struct OAuthService {
    oauth_client: Arc<HashMap<SupportIdp, Box<dyn OAuthRequest>>>,
}

impl OAuthService {
    pub fn new(cfg: &Config) -> Self {
        let mut clients = HashMap::with_capacity(1);
        let github_client = Box::new(GithubOAuthClient::from(&cfg.oidc.github));
        clients.insert(SupportIdp::Github, github_client as _);
        Self {
            oauth_client: Arc::new(clients),
        }
    }

    fn get_oauth_client(
        &self,
        idp: SupportIdp,
    ) -> Result<&Box<dyn OAuthRequest>, OAuthServiceError> {
        self.oauth_client
            .get(&idp)
            .ok_or(OAuthServiceError::NotSupportedIdp)
    }

    pub async fn login_call(
        &self,
        idp: SupportIdp,
    ) -> Result<(Url, AuthSession), OAuthServiceError> {
        let oauth_client = self.get_oauth_client(idp)?;
        Ok(oauth_client
            .login()
            .await
            .map_err(OAuthServiceError::from)?)
    }

    pub async fn callback_call(
        &self,
        idp: SupportIdp,
        code: String,
        pkce_verifier: String,
    ) -> Result<OAuthAccessToken, OAuthServiceError> {
        let oauth_client = self.get_oauth_client(idp)?;
        Ok(oauth_client
            .callback(code, pkce_verifier)
            .await
            .map_err(OAuthServiceError::from)?)
    }

    pub async fn get_user_id_call(
        &self,
        idp: SupportIdp,
        access_token: String,
    ) -> Result<String, OAuthServiceError> {
        let oauth_client = self.get_oauth_client(idp)?;
        let user_id = oauth_client.get_user_id(access_token).await?;
        Ok(user_id)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum OAuthServiceError {
    #[error("not supported oauth provider")]
    NotSupportedIdp,

    #[error("oauth login fail : {0}")]
    OAuthLoginFail(#[from] SAUOAuthDomainError),
}
