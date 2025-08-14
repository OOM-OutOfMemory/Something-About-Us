use crate::{
    application::service::{
        jwt_service::JwtService, oauth_service::OAuthService, user_service::UserService,
    },
    domain::oauth::sau_jwt_issuer::SAUJwtIssuer,
    infrastructure::{
        cache::memcached::repository::CacheRepoMchd,
        persistence::postgres::repository::DatabaseRepoPg,
    },
    interface::web::state::auth_session_cookie::AuthSessionCookieManager,
};

pub mod auth_session_cookie;
pub mod from_part;

#[derive(Clone)]
pub struct AppState {
    pub database_repo: DatabaseRepoPg,
    pub cache_repo: CacheRepoMchd,
    pub user_service: UserService<DatabaseRepoPg>,
    pub oauth_service: OAuthService,
    pub jwt_service: JwtService<SAUJwtIssuer>,
    pub auth_cookie_manager: AuthSessionCookieManager,
}
