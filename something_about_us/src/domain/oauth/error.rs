use thiserror::Error;

#[derive(Debug, Error)]
pub enum SAUOAuthDomainError {
    #[error("invalid issuer: {0}")]
    InvalidIssuer(String),

    #[error("invalid audience: {0}")]
    InvalidAudience(String),

    #[error("invalid url: {0}")]
    InvalidUrl(String),

    #[error("login failed: {0}")]
    LoginFailed(String),

    #[error("callback failed: {0}")]
    CallBackFailed(String),

    #[error("user info fetch failed : {0}")]
    UserInfoFetchFailed(String),

    #[error("jwt issue failed: {0}")]
    JwtIssueFailed(String),
}

#[cfg(test)]
mod tests {
    include!("error_test.rs");
}
