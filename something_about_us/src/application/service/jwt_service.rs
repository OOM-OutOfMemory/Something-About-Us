use std::{ops::Deref, sync::Arc};

use jsonwebtoken::jwk::JwkSet;
use uuid::Uuid;

use crate::domain::oauth::{sau_jwt::SAUJwt, sau_jwt_issuer::JwtIssue};

#[derive(Clone)]
pub struct JwtService<I: JwtIssue> {
    jwks: Arc<JwkSet>,
    current_kid: Uuid,
    jwt_issuer: Arc<I>,
}

impl<I: JwtIssue> JwtService<I> {
    pub fn new(jwt_issuer: Arc<I>, kid: Uuid) -> Self {
        let jwks = Arc::new(jwt_issuer.create_jwks());
        Self {
            jwt_issuer,
            jwks,
            current_kid: kid,
        }
    }

    pub fn get_jwks(&self) -> JwkSet {
        self.jwks.clone().deref().clone()
    }

    pub fn issue_with_id(&self, uid: &Uuid) -> Result<SAUJwt, JwtIssuerServiceError> {
        Ok(self
            .jwt_issuer
            .issue_with_id(&self.current_kid, uid)
            .map_err(|e| JwtIssuerServiceError::JwtIssueError(e.to_string()))?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum JwtIssuerServiceError {
    #[error("jwt issue error : {0}")]
    JwtIssueError(String),
}
