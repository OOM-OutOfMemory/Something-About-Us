use axum::{routing::get, Router};

use crate::interface::web::{
    state::AppState,
    v1::oauth::{callback::callback, login::login},
};

pub mod callback;
pub mod login;

pub async fn router(state: AppState) -> Router {
    Router::new()
        .route("/{idp}/login", get(login))
        .route("/{idp}/callback", get(callback))
        .with_state(state)
}
