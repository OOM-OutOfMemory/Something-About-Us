#![allow(dead_code)]

use utoipa::OpenApi;

use crate::interface::web::{
    dto::error_response::ErrorResponse,
    v1::{
        health::gen_openapi_health,
        jwks::gen_openapi_jwks,
        oauth::{callback::gen_openapi_callback, login::gen_openapi_login},
    },
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Something About Us API",
        version = "0.1.0",
        description = "OpenAPI documentation for the Something About Us service."
    ),
    components(
        schemas(
            ErrorResponse,
        ),
    ),
    tags(
        (name = "Heartbeat", description = "Health check endpoints"),
        (name = "OAuth", description = "OAuth 2.0 login flow"),
        (name = "JWKS", description = "JSON Web Key Set endpoints")
    )
)]
pub struct ApiDoc;

pub fn gen_openapi() -> utoipa::openapi::OpenApi {
    let mut docs = ApiDoc::openapi();

    docs.merge(gen_openapi_health());
    docs.merge(gen_openapi_callback());
    docs.merge(gen_openapi_login());
    docs.merge(gen_openapi_jwks());

    docs
}
