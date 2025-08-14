use axum::{
    extract::{rejection::PathRejection, Path, State},
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::CookieJar;

use crate::{
    application::{
        port::auth_session_repository::AuthSessionCacheRepo, service::oauth_service::OAuthService,
    },
    domain::idp::supported_idp::SupportIdp,
    infrastructure::{
        cache::memcached::repository::CacheRepoMchd, cookie::AuthSessionCookieIssuer,
    },
    interface::web::{error::WebError, state::auth_session_cookie::AuthSessionCookieManager},
};

pub async fn login(
    path: Result<Path<SupportIdp>, PathRejection>,
    State(oauth_service): State<OAuthService>,
    State(cache_service): State<CacheRepoMchd>,
    State(auth_cookie_manager): State<AuthSessionCookieManager>,
    cookie_jar: CookieJar,
) -> Result<Response, WebError> {
    let Path(idp) = path?;

    let (authenticate_redirect_url, auth_session_info) = oauth_service
        .login_call(idp)
        .await
        .map_err(|e| WebError::Auth(e.to_string()))?;

    cache_service
        .set_auth_session(&auth_session_info)
        .await
        .map_err(|e| WebError::InternalServerError(e.to_string()))?;

    let auth_session_cookie = auth_cookie_manager.issuer_auth_session_cookie(auth_session_info.id);
    let cookie_jar = cookie_jar.add(auth_session_cookie);

    Ok((
        cookie_jar,
        Redirect::to(authenticate_redirect_url.to_string().as_str()),
    )
        .into_response())
}
