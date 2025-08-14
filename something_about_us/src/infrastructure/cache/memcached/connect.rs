use anyhow::{Context, Result};
use deadpool::managed::Pool;
use deadpool_memcached::Manager;

use crate::infrastructure::config::types::MemCachedConfig;

pub async fn memcached_connect(config: &MemCachedConfig) -> Result<Pool<Manager>> {
    let addr = format!(
        "{}:{}",
        config.connect_info.address, config.connect_info.port
    );
    let manager = Manager::new(addr.clone());
    let memecached_config =
        deadpool_memcached::PoolConfig::new(config.runtime_options.pool_size as usize);

    let pool: Pool<Manager> = Pool::builder(manager)
        .config(memecached_config)
        .build()
        .context("fail to make memecached connection pool")?;

    if config.runtime_options.init_flush {
        let mut client = async_memcached::Client::new(addr)
            .await
            .context("fail to make memcached client for flushing")?;
        client.flush_all().await.context("fail to flushing")?;
    }

    Ok(pool)
}
