use crate::domain::oauth::{error::SAUOAuthDomainError, sau_jwt::SAUJwt};
use jsonwebtoken::{jwk::JwkSet, DecodingKey, EncodingKey};
use std::{collections::HashMap, sync::Arc, time::Duration};
use uuid::Uuid;

#[derive(Clone)]
pub struct SAUJwtIssuer {
    pub header: jsonwebtoken::Header,
    pub iss: String,
    pub aud: String,
    pub access_token_ttl: Duration,
    pub key_pair: Arc<HashMap<Uuid, KeyPair>>,
}

pub struct KeyPair {
    pub private_key: EncodingKey,
    pub public_key: DecodingKey,
    pub x: String,
}

impl SAUJwtIssuer {
    pub fn new(
        iss: String,
        aud: String,
        access_token_ttl: u64,
        key_pair: HashMap<Uuid, KeyPair>,
    ) -> Self {
        let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::EdDSA);
        Self::validate_issuer(&iss).expect("validation error");
        Self::validate_audience(&aud).expect("validation error");
        Self {
            header,
            iss,
            aud,
            access_token_ttl: Duration::from_secs(access_token_ttl),
            key_pair: Arc::new(key_pair),
        }
    }

    pub fn validate(&self) -> Result<(), SAUOAuthDomainError> {
        Self::validate_issuer(&self.iss)?;
        Self::validate_audience(&self.aud)?;
        Ok(())
    }

    pub fn validate_issuer(iss: &str) -> Result<(), SAUOAuthDomainError> {
        if iss.is_empty() || iss.len() > 50 {
            return Err(SAUOAuthDomainError::InvalidIssuer(iss.to_string()));
        }
        Ok(())
    }

    pub fn validate_audience(aud: &str) -> Result<(), SAUOAuthDomainError> {
        if aud.is_empty() || aud.len() > 50 {
            return Err(SAUOAuthDomainError::InvalidAudience(aud.to_string()));
        }
        Ok(())
    }
}

pub trait JwtIssue {
    fn issue_with_id(&self, kid: &Uuid, uid: &Uuid) -> Result<SAUJwt, SAUOAuthDomainError>;
    fn create_jwks(&self) -> JwkSet;
}

// JwtIssue implementation is provided in infrastructure/auth/jwt_issuer_helper.rs

#[cfg(test)]
mod tests {
    include!("sau_jwt_issuer_test.rs");
}
