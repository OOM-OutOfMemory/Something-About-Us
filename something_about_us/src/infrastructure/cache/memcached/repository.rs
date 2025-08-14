use deadpool::managed::{Object, Pool};
use deadpool_memcached::Manager;

use crate::application::port::auth_session_repository::AuthSessionCacheRepoError;

pub mod auth_session_repo;

#[derive(Clone)]
pub struct CacheRepoMchd {
    conn: Pool<Manager>,
}

impl CacheRepoMchd {
    pub fn new(conn: Pool<Manager>) -> Self {
        Self { conn }
    }

    pub async fn get(&self) -> Result<Object<Manager>, AuthSessionCacheRepoError> {
        self.conn
            .get()
            .await
            .map_err(|e| AuthSessionCacheRepoError::CacheConnectionError(e.to_string()))
    }
}
