use chrono::Utc;
use sonic_rs::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

use crate::domain::{idp::supported_idp::SupportIdp, user::error::SAUUserDomainError};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SAUUser {
    pub id: Uuid,
    pub username: Option<Username>,
    pub email: Option<Email>,
    pub idp: SupportIdp,
    pub idp_uid: String,
    pub is_active: bool,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Username(String);

impl Username {
    pub fn new(username: String) -> Result<Self, SAUUserDomainError> {
        if username.is_empty() || username.len() > 50 {
            Err(SAUUserDomainError::InvalidUsername(username))
        } else {
            Ok(Self(username))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub fn new(email: String) -> Result<Self, SAUUserDomainError> {
        if email.is_empty() || !email.contains('@') || email.len() > 254 {
            Err(SAUUserDomainError::InvalidEmail(email))
        } else {
            Ok(Self(email))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
