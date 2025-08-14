use axum::{
    extract::rejection::{JsonRejection, PathRejection, QueryRejection},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use tracing::{info, warn};

use crate::interface::web::dto::error_response::ErrorResponse;

#[derive(Debug, thiserror::Error)]
pub enum WebError {
    #[error("path extraction error")]
    Path(#[from] PathRejection),
    #[error("query extraction error")]
    Query(#[from] QueryRejection),
    #[error("json extraction error")]
    Json(#[from] JsonRejection),

    #[error("auth error")]
    Auth(String),

    #[error("internal server error")]
    InternalServerError(String),
}

impl IntoResponse for WebError {
    fn into_response(self) -> axum::response::Response {
        match &self {
            WebError::Path(inner_error) => {
                info!("{:?} : {:?}", self, inner_error);
                (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        code: "INVALID INPUT".to_string(),
                        message: self.to_string(),
                        details: Some(inner_error.to_string()),
                    }),
                )
            }
            WebError::Query(inner_error) => {
                info!("{:?} : {:?}", self, inner_error);
                (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        code: "INVALID INPUT".to_string(),
                        message: self.to_string(),
                        details: Some(inner_error.to_string()),
                    }),
                )
            }
            WebError::Json(inner_error) => {
                info!("{:?} : {:?}", self, inner_error);
                (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        code: "INVALID INPUT".to_string(),
                        message: self.to_string(),
                        details: Some(inner_error.to_string()),
                    }),
                )
            }
            WebError::Auth(inner_error) => {
                info!("{:?} : {:?}", self, inner_error);
                (
                    StatusCode::UNAUTHORIZED,
                    Json(ErrorResponse {
                        code: "UNAUTHORIZED".to_string(),
                        message: self.to_string(),
                        details: Some(inner_error.to_string()),
                    }),
                )
            }
            WebError::InternalServerError(inner_error) => {
                warn!("{:?} : {:?}", self, inner_error);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        code: "INTERNAL SERVER ERROR".to_string(),
                        message: self.to_string(),
                        details: Some(inner_error.to_string()),
                    }),
                )
            }
        }
        .into_response()
    }
}
