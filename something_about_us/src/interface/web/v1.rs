use axum::Router;

use crate::interface::web::state::AppState;

pub mod health;
pub mod jwks;
pub mod oauth;

pub mod health_test;

pub async fn router(state: AppState) -> Router {
    Router::new().nest(
        "/v1",
        Router::new()
            .nest("/heartbeat", health::router().await)
            .nest("/oauth", oauth::router(state.clone()).await)
            .nest("/jwks", jwks::router(state.clone()).await),
    )
}
