use axum::{routing::get, Router};

async fn health_check() -> &'static str {
    "Ok - Something About Us"
}

pub async fn router() -> Router {
    Router::new().route("/", get(health_check))
}
