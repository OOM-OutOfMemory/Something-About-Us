use anyhow::{Context, Result};
use sea_orm::DatabaseConnection;
use std::time::Duration;

use crate::infrastructure::config::types::PostgresConfig;

pub async fn postgres_connect(config: &PostgresConfig) -> Result<DatabaseConnection> {
    let connect_info = &config.connect_info;
    let database_url = url::Url::parse(
        format!(
            "postgres://{}:{}@{}:{}/{}",
            connect_info.username,
            connect_info.password,
            connect_info.address,
            connect_info.port,
            connect_info.db_name
        )
        .as_str(),
    )?;

    let runtime_options = &config.runtime_options;
    let mut opt = sea_orm::ConnectOptions::new(database_url.to_string());
    opt.min_connections(runtime_options.min_pool_size)
        .connect_timeout(Duration::from_secs(runtime_options.connect_timeout))
        .acquire_timeout(Duration::from_secs(runtime_options.acquire_timeout))
        .idle_timeout(Duration::from_secs(runtime_options.idle_timeout))
        .max_lifetime(Duration::from_secs(runtime_options.max_lifetime))
        .sqlx_logging(runtime_options.sqlx_logging);

    let db = sea_orm::Database::connect(opt)
        .await
        .context("fail to database connection")?;

    Ok(db)
}
