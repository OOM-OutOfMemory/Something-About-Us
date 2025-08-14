use crate::domain::{idp::supported_idp::SupportIdp, user::sau_user::SAUUser};

#[async_trait::async_trait]
pub trait SAUUserRepo: Send + Sync {
    async fn get_user_by_idp_and_idp_id(
        &self,
        idp: &SupportIdp,
        idp_id: &String,
    ) -> Result<Option<SAUUser>, SAUUserRepoError>;

    async fn create_user_by_idp_and_idp_id(
        &self,
        idp: &SupportIdp,
        idp_id: &String,
    ) -> Result<SAUUser, SAUUserRepoError>;
}

#[derive(thiserror::Error, Debug)]
pub enum SAUUserRepoError {
    #[error("database error : {0}")]
    DatabaseError(String),

    #[error("casting error : {0}")]
    CastingError(String),
}
