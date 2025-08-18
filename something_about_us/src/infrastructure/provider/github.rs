use oauth2::{
    basic::BasicClient, AuthUrl, Client, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use reqwest::redirect::Policy;
use sonic_rs::Deserialize;
use std::sync::Arc;
use tracing::warn;
use url::Url;
use uuid::Uuid;

use crate::{
    domain::oauth::{
        auth_session::{AuthSession, AUTH_HTTP_AGENT_NAME},
        error::SAUOAuthDomainError,
        oauth_provider::OAuthRequest,
        sau_jwt::OAuthAccessToken,
    },
    infrastructure::config::types::GithubConfig,
};

type GithubClient = Arc<
    Client<
        oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
        oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>,
        oauth2::StandardTokenIntrospectionResponse<
            oauth2::EmptyExtraTokenFields,
            oauth2::basic::BasicTokenType,
        >,
        oauth2::StandardRevocableToken,
        oauth2::StandardErrorResponse<oauth2::RevocationErrorResponseType>,
        oauth2::EndpointSet,
        oauth2::EndpointNotSet,
        oauth2::EndpointNotSet,
        oauth2::EndpointNotSet,
        oauth2::EndpointSet,
    >,
>;

#[derive(Clone)]
pub struct GithubOAuthClient {
    auth_client: GithubClient,
    resource_endpoint: Url,
    auth_callback_http_client: reqwest::Client,
    resource_http_request_client: reqwest::Client,
}

impl From<&GithubConfig> for GithubOAuthClient {
    fn from(value: &GithubConfig) -> Self {
        let idp_secret = ClientSecret::new(value.client_secret.clone());
        let idp_id = ClientId::new(value.client_id.clone());
        let auth_url = AuthUrl::new(value.auth_url.clone())
            .expect("invalid github authorization endpoint url");
        let token_url =
            TokenUrl::new(value.token_url.clone()).expect("invalid github token endpoint url");
        let redirect_url = value.redirect_url.clone();

        let auth_client = BasicClient::new(idp_id)
            .set_client_secret(idp_secret)
            .set_auth_uri(auth_url)
            .set_token_uri(token_url)
            .set_redirect_uri(RedirectUrl::from_url(redirect_url));

        let resource_http_request_client = reqwest::Client::new();
        let auth_callback_http_client = reqwest::Client::builder()
            .redirect(Policy::none())
            .build()
            .expect("github callback http client init failed");
        let resource_endpoint =
            Url::parse(value.resource_url.as_str()).expect("invalid github resource  endpoint url");

        Self {
            auth_client: Arc::new(auth_client),
            resource_endpoint,
            resource_http_request_client,
            auth_callback_http_client,
        }
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct GithubApiUserResponse {
    pub id: i64,
    pub login: String,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub user_type: String,
    pub user_view_type: String,
    pub site_admin: bool,
    pub name: Option<String>,
    pub company: Option<String>,
    pub blog: String,
    pub location: Option<String>,
    pub email: Option<String>,
    pub hireable: Option<bool>,
    pub bio: Option<String>,
    pub twitter_username: Option<String>,
    pub notification_email: Option<String>,
    pub public_repos: i32,
    pub public_gists: i32,
    pub followers: i32,
    pub following: i32,
    pub created_at: String,
    pub updated_at: String,
    pub private_gists: i32,
    pub total_private_repos: i32,
    pub owned_private_repos: i32,
    pub disk_usage: i32,
    pub collaborators: i32,
    pub two_factor_authentication: bool,
    pub plan: GithubPlan,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct GithubPlan {
    pub name: String,
    pub space: i64,
    pub collaborators: i32,
    pub private_repos: i32,
}

#[async_trait::async_trait]
impl OAuthRequest for GithubOAuthClient {
    async fn login(&self) -> Result<(Url, AuthSession), SAUOAuthDomainError> {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let (auth_url, csrf_token) = self
            .auth_client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("read:user".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        let auth_session = AuthSession {
            id: Uuid::now_v7(),
            pkce_verifier: pkce_verifier.secret().to_string(),
            csrf_token: csrf_token.secret().to_string(),
        };

        Ok((auth_url, auth_session))
    }

    async fn callback(
        &self,
        code: String,
        pkce_verifier: String,
    ) -> Result<OAuthAccessToken, SAUOAuthDomainError> {
        let resp = self
            .auth_client
            .exchange_code(oauth2::AuthorizationCode::new(code))
            .set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier))
            .request_async(&self.auth_callback_http_client)
            .await
            .map_err(|e| SAUOAuthDomainError::CallBackFailed(e.to_string()))?;

        Ok(resp.access_token().secret().to_string())
    }
    async fn get_user_id(
        &self,
        access_token: OAuthAccessToken,
    ) -> Result<String, SAUOAuthDomainError> {
        let user_info_url = self.resource_endpoint.join("user").map_err(|e| {
            SAUOAuthDomainError::InvalidUrl(format!("for user api : {}", e.to_string()))
        })?;
        let response = self
            .resource_http_request_client
            .get(user_info_url)
            .bearer_auth(access_token)
            .header(reqwest::header::USER_AGENT, AUTH_HTTP_AGENT_NAME)
            .send()
            .await
            .map_err(|e| SAUOAuthDomainError::UserInfoFetchFailed(e.to_string()))?;

        if !response.status().is_success() {
            return Err(SAUOAuthDomainError::UserInfoFetchFailed(
                response.status().to_string(),
            ));
        }

        let body = response
            .json::<GithubApiUserResponse>()
            .await
            .map_err(|e| {
                warn!("{:?}", e);
                SAUOAuthDomainError::UserInfoFetchFailed(e.to_string())
            })?;

        Ok(body.id.to_string())
    }
}
