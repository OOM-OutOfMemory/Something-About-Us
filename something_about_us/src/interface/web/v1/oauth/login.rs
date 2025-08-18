use axum::{
    extract::{rejection::PathRejection, Path, State},
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::CookieJar;
use utoipa::OpenApi;

use crate::{
    application::{
        port::auth_session_repository::AuthSessionCacheRepo, service::oauth_service::OAuthService,
    },
    domain::idp::supported_idp::SupportIdp,
    infrastructure::{
        cache::memcached::repository::CacheRepoMchd, cookie::AuthSessionCookieIssuer,
    },
    interface::web::{
        dto::idp_path::IdpPathParam, error::WebError,
        state::auth_session_cookie::AuthSessionCookieManager,
    },
};

#[utoipa::path(
    get,
    path = "/api/v1/oauth/{idp}/login",
    tag = "OAuth",
    operation_id = "oauthLogin",
    params(IdpPathParam),
    responses(
        (status = 302, description = "Redirect to Identity Provider for authentication",
            headers(
                ("Set-Cookie" = String, description = "Auth session cookie"),
                ("Location" = String, description = "Redirect target URL to the IdP")
            )
        ),
        (status = 500, description = "Internal Server Error", body = crate::interface::web::dto::error_response::ErrorResponse)
    )
)]
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

#[derive(OpenApi)]
#[openapi(paths(login), components(schemas(IdpPathParam)))]
struct LoginOpenApi;

pub fn gen_openapi_login() -> utoipa::openapi::OpenApi {
    LoginOpenApi::openapi()
}
