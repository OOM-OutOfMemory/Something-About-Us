use std::sync::Arc;

use axum::Router;

use crate::{
    application::service::{
        jwt_service::JwtService, oauth_service::OAuthService, user_service::UserService,
    },
    infrastructure::{
        self,
        auth::jwt_issuer_helper::JwtIssuerHelper,
        cache::memcached::{connect::memcached_connect, repository::CacheRepoMchd},
        config::validation::check_config_validation,
        persistence::postgres::{connect::postgres_connect, repository::DatabaseRepoPg},
    },
    interface::web::{
        server::make_router,
        state::{auth_session_cookie::AuthSessionCookieManager, AppState},
    },
};

pub async fn test_make_route() -> Router {
    let cfg = infrastructure::config::read::read_config()
        .and_then(check_config_validation)
        .unwrap();

    // infra
    let postgres_connection_pool = postgres_connect(&cfg.postgres).await.unwrap();
    let memcached_connection_pool = memcached_connect(&cfg.memcached).await.unwrap();

    // repo
    let database_repo = DatabaseRepoPg::new(postgres_connection_pool);
    let cache_repo = CacheRepoMchd::new(memcached_connection_pool);

    // service
    let jwt_issuer = Arc::new(JwtIssuerHelper::make_jwtissuer(&cfg.jwt).await);
    let jwt_service = JwtService::new(jwt_issuer, cfg.jwt.keys[0].kid.clone());
    let user_service = UserService::new(database_repo.clone());
    let oauth_service = OAuthService::new(&cfg);

    // http cookie
    let auth_cookie_manager = AuthSessionCookieManager::from(&cfg.security.session);

    // http server state
    let http_server_state = AppState {
        database_repo,
        cache_repo,
        user_service,
        oauth_service,
        jwt_service,
        auth_cookie_manager,
    };

    make_router(http_server_state).await
}
