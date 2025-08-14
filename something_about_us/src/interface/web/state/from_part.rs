use axum::extract::FromRef;

use crate::{
    application::service::{
        jwt_service::JwtService, oauth_service::OAuthService, user_service::UserService,
    },
    domain::oauth::sau_jwt_issuer::SAUJwtIssuer,
    infrastructure::{
        cache::memcached::repository::CacheRepoMchd,
        persistence::postgres::repository::DatabaseRepoPg,
    },
    interface::web::state::{auth_session_cookie::AuthSessionCookieManager, AppState},
};

impl FromRef<AppState> for DatabaseRepoPg {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.database_repo.clone()
    }
}

impl FromRef<AppState> for CacheRepoMchd {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.cache_repo.clone()
    }
}

impl FromRef<AppState> for UserService<DatabaseRepoPg> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.user_service.clone()
    }
}

impl FromRef<AppState> for OAuthService {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.oauth_service.clone()
    }
}

impl FromRef<AppState> for JwtService<SAUJwtIssuer> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.jwt_service.clone()
    }
}

impl FromRef<AppState> for AuthSessionCookieManager {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.auth_cookie_manager.clone()
    }
}
