#[derive(thiserror::Error, Debug)]
pub enum SAUUserDomainError {
    #[error("Invalid email format : {0}")]
    InvalidEmail(String),

    #[error("Invalid username : {0}")]
    InvalidUsername(String),
}
