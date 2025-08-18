use axum::{routing::get, Router};
use utoipa::OpenApi;

#[utoipa::path(
    get,
    path = "/api/v1/heartbeat",
    tag = "Heartbeat",
    operation_id = "healthCheck",
    responses(
        (status = 200, description = "Service is healthy", content_type = "text/plain", body = String)
    )
)]
async fn health_check() -> &'static str {
    "Ok - Something About Us"
}

pub async fn router() -> Router {
    Router::new().route("/", get(health_check))
}

#[derive(OpenApi)]
#[openapi(paths(health_check))]
struct HealthCheckOpenApi;

pub fn gen_openapi_health() -> utoipa::openapi::OpenApi {
    HealthCheckOpenApi::openapi()
}
