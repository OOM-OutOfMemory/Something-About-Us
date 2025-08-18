use sea_orm::{ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, TryInsertResult};
use uuid::Uuid;

use crate::{
    application::port::sau_user_repository::{SAUUserRepo, SAUUserRepoError},
    domain::{
        idp::supported_idp::SupportIdp,
        user::sau_user::{Email, SAUUser, Username},
    },
    infrastructure::persistence::postgres::{entity::users, repository::DatabaseRepoPg},
};

#[async_trait::async_trait]
impl SAUUserRepo for DatabaseRepoPg {
    async fn get_user_by_idp_and_idp_id(
        &self,
        idp: &SupportIdp,
        idp_id: &String,
    ) -> Result<Option<SAUUser>, SAUUserRepoError> {
        users::Entity::find()
            .filter(
                users::Column::Idp
                    .eq(idp.as_str())
                    .and(users::Column::IdpUid.eq(idp_id)),
            )
            .one(&self.conn)
            .await
            .map_err(|e| SAUUserRepoError::DatabaseError(e.to_string()))?
            .map(SAUUser::try_from)
            .transpose()
    }

    async fn create_user_by_idp_and_idp_id(
        &self,
        idp: &SupportIdp,
        idp_id: &String,
    ) -> Result<SAUUser, SAUUserRepoError> {
        let now = chrono::Utc::now().into();
        let new_user = users::ActiveModel {
            id: Set(Uuid::now_v7()),
            username: Set(None),
            email: Set(None),
            is_active: Set(true),
            idp: Set(idp.as_str().to_string()),
            idp_uid: Set(idp_id.clone()),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let result = users::Entity::insert(new_user)
            .do_nothing()
            .exec_with_returning(&self.conn)
            .await
            .map_err(|e| SAUUserRepoError::DatabaseError(e.to_string()))?;

        if let TryInsertResult::Inserted(model) = result {
            return SAUUser::try_from(model)
                .map_err(|e| SAUUserRepoError::CastingError(e.to_string()));
        }

        let search = self.get_user_by_idp_and_idp_id(idp, idp_id).await?;

        match search {
            Some(user) => Ok(user),
            None => Err(SAUUserRepoError::DatabaseError(
                "failed to create user".to_string(),
            )),
        }
    }
}

impl TryFrom<users::Model> for SAUUser {
    type Error = SAUUserRepoError;

    fn try_from(value: users::Model) -> Result<Self, Self::Error> {
        let username = value
            .username
            .map(Username::new)
            .transpose()
            .map_err(|e| SAUUserRepoError::CastingError(e.to_string()))?;

        let email = value
            .email
            .map(Email::new)
            .transpose()
            .map_err(|e| SAUUserRepoError::CastingError(e.to_string()))?;

        let idp = SupportIdp::try_from(value.idp.as_str())
            .map_err(|e| SAUUserRepoError::CastingError(e.to_string()))?;

        Ok(Self {
            id: value.id,
            username,
            email,
            idp,
            idp_uid: value.idp_uid,
            is_active: value.is_active,
            created_at: value.created_at.into(),
            updated_at: value.updated_at.into(),
        })
    }
}
