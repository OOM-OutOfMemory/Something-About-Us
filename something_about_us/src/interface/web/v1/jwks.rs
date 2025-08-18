use axum::{
    extract::State,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use utoipa::OpenApi;

use crate::{
    application::service::jwt_service::JwtService,
    domain::oauth::sau_jwt_issuer::SAUJwtIssuer,
    interface::web::{dto::jwks_response::JwkSetDoc, error::WebError, state::AppState},
};

#[utoipa::path(
    get,
    path = "/api/v1/jwks",
    tag = "JWKS",
    operation_id = "getJwks",
    responses(
        (status = 200, description = "JSON Web Key Set used for verifying JWTs", body = JwkSetDoc)
    )
)]
async fn jwks(State(jwt_service): State<JwtService<SAUJwtIssuer>>) -> Result<Response, WebError> {
    let jwks = jwt_service.get_jwks();
    Ok(Json(jwks).into_response())
}

pub async fn router(state: AppState) -> Router {
    Router::new().route("/", get(jwks)).with_state(state)
}

#[derive(OpenApi)]
#[openapi(paths(jwks), components(schemas(JwkSetDoc)))]
struct JwksOpenApi;

pub fn gen_openapi_jwks() -> utoipa::openapi::OpenApi {
    JwksOpenApi::openapi()
}
