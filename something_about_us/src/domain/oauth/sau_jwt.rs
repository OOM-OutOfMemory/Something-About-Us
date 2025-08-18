use sonic_rs::{Deserialize, Serialize};
use uuid::Uuid;

pub type OAuthAccessToken = String;
pub type SAUJwt = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct SAUClaims {
    pub aud: String,
    pub iss: String,
    pub sub: Uuid,
    pub exp: i64,
    pub jti: Uuid,
    pub iat: i64,
    pub nbf: i64,
}

#[cfg(test)]
mod tests {
    include!("sau_jwt_test.rs");
}
