use anyhow::{Context, Result};
use axum::Router;
use tokio::net::TcpListener;
use tracing::info;

use crate::interface::web::{state::AppState, v1};

pub async fn server_run(addr: String, port: u16, state: AppState) -> Result<()> {
    let addr_str = format!("{}:{}", addr, port);
    let listener = TcpListener::bind(addr_str.clone())
        .await
        .context(format!("fail to listen : {}", addr_str))?;
    let router = make_router(state).await;

    info!("server is running");
    Ok(axum::serve(listener, router.into_make_service())
        .await
        .context("fail to run server")?)
}

pub async fn make_router(state: AppState) -> Router {
    Router::new().nest("/api", v1::router(state).await)
}
