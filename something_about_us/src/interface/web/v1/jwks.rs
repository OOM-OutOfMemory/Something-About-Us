use axum::{
    extract::State,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};

use crate::{
    application::service::jwt_service::JwtService,
    domain::oauth::sau_jwt_issuer::SAUJwtIssuer,
    interface::web::{error::WebError, state::AppState},
};

async fn jwks(State(jwt_service): State<JwtService<SAUJwtIssuer>>) -> Result<Response, WebError> {
    let jwks = jwt_service.get_jwks();
    Ok(Json(jwks).into_response())
}

pub async fn router(state: AppState) -> Router {
    Router::new().route("/", get(jwks)).with_state(state)
}
