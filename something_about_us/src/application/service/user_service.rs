use crate::{
    application::port::sau_user_repository::SAUUserRepo,
    domain::{idp::supported_idp::SupportIdp, user::sau_user::SAUUser},
};

#[derive(Clone)]
pub struct UserService<U: SAUUserRepo> {
    user_repo: U,
}

// #[async_trait::async_trait]
impl<U: SAUUserRepo> UserService<U> {
    pub fn new(user_repo: U) -> Self {
        Self { user_repo }
    }

    pub async fn get_or_create_user_from_callback(
        &self,
        idp: SupportIdp,
        idp_id: String,
    ) -> Result<SAUUser, UserServiceError> {
        let user = self
            .user_repo
            .get_user_by_idp_and_idp_id(&idp, &idp_id)
            .await
            .map_err(|e| UserServiceError::UserFetch(e.to_string()))?;
        match user {
            Some(value) => Ok(value),
            None => {
                let new_user = self
                    .user_repo
                    .create_user_by_idp_and_idp_id(&idp, &idp_id)
                    .await
                    .map_err(|e| UserServiceError::UserCreate(e.to_string()))?;
                Ok(new_user)
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum UserServiceError {
    #[error("user service fetch error : {0}")]
    UserFetch(String),

    #[error("user service create error : {0}")]
    UserCreate(String),
}
