use sea_orm::DatabaseConnection;

pub mod user_repo;

#[derive(Clone)]
pub struct DatabaseRepoPg {
    conn: DatabaseConnection,
}

impl DatabaseRepoPg {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}
