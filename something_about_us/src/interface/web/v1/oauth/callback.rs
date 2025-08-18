use axum::{
    extract::{
        rejection::{PathRejection, QueryRejection},
        Path, Query, State,
    },
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;
use utoipa::OpenApi;
use uuid::Uuid;

use crate::{
    application::{
        port::auth_session_repository::AuthSessionCacheRepo,
        service::{
            jwt_service::JwtService, oauth_service::OAuthService, user_service::UserService,
        },
    },
    domain::{
        idp::supported_idp::SupportIdp,
        oauth::{auth_session::AUTH_SESSION_COOKIE_NAME, sau_jwt_issuer::SAUJwtIssuer},
    },
    infrastructure::{
        cache::memcached::repository::CacheRepoMchd,
        persistence::postgres::repository::DatabaseRepoPg,
    },
    interface::web::{
        dto::{
            callback_param::OAuthCallbackQuery, error_response::ErrorResponse,
            idp_path::IdpPathParam, jwt_response::Token,
        },
        error::WebError,
    },
};

#[utoipa::path(
    get,
    path = "/api/v1/oauth/{idp}/callback",
    tag = "OAuth",
    operation_id = "oauthCallback",
    params(
        IdpPathParam,
        OAuthCallbackQuery
    ),
    responses(
        (status = 200, description = "Login complete. JWT access token issued", body = Token),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    )
)]
pub async fn callback(
    path: Result<Path<SupportIdp>, PathRejection>,
    query: Result<Query<OAuthCallbackQuery>, QueryRejection>,
    State(oauth_service): State<OAuthService>,
    State(cache_service): State<CacheRepoMchd>,
    State(user_service): State<UserService<DatabaseRepoPg>>,
    State(jwt_issuer): State<JwtService<SAUJwtIssuer>>,
    cookie_jar: CookieJar,
) -> Result<Response, WebError> {
    let Path(idp) = path?;
    let Query(callback_params) = query?;

    let session_cookie = cookie_jar
        .get(AUTH_SESSION_COOKIE_NAME)
        .ok_or_else(|| WebError::Auth("session cookie is not found".to_string()))?;
    let session_id = session_cookie
        .value()
        .parse::<Uuid>()
        .map_err(|e| WebError::Auth(format!("invalid session_id format : {}", e.to_string())))?;
    let cookie_jar = cookie_jar.clone().remove(session_cookie.clone());

    let auth_session_info = cache_service
        .get_auth_session(session_id)
        .await
        .map_err(|e| WebError::InternalServerError(e.to_string()))?;
    if auth_session_info.csrf_token != callback_params.state {
        return Err(WebError::Auth("csrf token is invalid".to_string()));
    }

    let access_token = oauth_service
        .callback_call(
            idp.clone(),
            callback_params.code,
            auth_session_info.pkce_verifier,
        )
        .await
        .map_err(|e| WebError::Auth(e.to_string()))?;
    let idp_user_id = oauth_service
        .get_user_id_call(idp.clone(), access_token)
        .await
        .map_err(|e| WebError::Auth(e.to_string()))?;

    let user = user_service
        .get_or_create_user_from_callback(idp, idp_user_id)
        .await
        .map_err(|e| WebError::InternalServerError(e.to_string()))?;

    let jwt = jwt_issuer.issue_with_id(&user.id).map_err(|e| {
        WebError::InternalServerError(format!("fail to issue jwt: {}", e.to_string()))
    })?;

    Ok((
        cookie_jar,
        (
            axum::http::StatusCode::OK,
            axum::Json(Token { access_token: jwt }),
        ),
    )
        .into_response())
}

#[derive(OpenApi)]
#[openapi(
    paths(callback),
    components(schemas(Token, IdpPathParam, OAuthCallbackQuery))
)]
struct CallbackOpenApi;

pub fn gen_openapi_callback() -> utoipa::openapi::OpenApi {
    CallbackOpenApi::openapi()
}
