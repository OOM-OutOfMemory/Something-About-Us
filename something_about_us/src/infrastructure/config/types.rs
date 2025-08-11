use sonic_rs::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub server: Server,
    pub logger: LoggerConfig,
    pub postgres: PostgresConfig,
    pub memcached: MemCachedConfig,
    pub jwks: JwksConfig,
    pub oidc: OIDCProviderConfig,
    pub security: SecurityConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Server {
    pub domain: String,
    pub port: u16,
    pub user_agent: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LoggerConfig {
    pub level: String,
}

#[derive(Deserialize, Debug)]
pub struct PostgresConfig {
    pub connect_info: PostgresConnectConfig,
    pub runtime_options: PostgresRuntimeConfig,
}

#[derive(Deserialize, Debug)]
pub struct PostgresConnectConfig {
    pub address: String,
    pub port: u32,
    pub username: String,
    pub password: String,
    pub db_name: String,
}

#[derive(Deserialize, Debug)]
pub struct PostgresRuntimeConfig {
    pub max_pool_size: u32,
    pub min_pool_size: u32,
    pub connect_timeout: u64,
    pub acquire_timeout: u64,
    pub idle_timeout: u64,
    pub max_lifetime: u64,
    pub sqlx_logging: bool,
    pub log_level: String,
}

#[derive(Deserialize, Debug)]
pub struct MemCachedConfig {
    pub connect_info: MemeCachedConnectConfig,
    pub runtime_options: MemCachedRuntimeConfig,
}

#[derive(Deserialize, Debug)]
pub struct MemeCachedConnectConfig {
    pub address: String,
    pub port: u32,
}

#[derive(Deserialize, Debug)]
pub struct MemCachedRuntimeConfig {
    pub init_flush: bool,
    pub pool_size: u32,
    pub read_timeout: u64,
    pub write_timeout: u64,
}

#[derive(Deserialize, Debug)]
pub struct JwksConfig {
    pub iss: String,
    pub aud: String,
    pub keys_path: String,
    pub keys: Vec<KeyConfig>,
}

#[derive(Deserialize, Debug)]
pub struct KeyConfig {
    pub kid: Uuid,
}

#[derive(Deserialize, Debug)]
pub struct OIDCProviderConfig {
    pub github: GithubConfig,
}

#[derive(Deserialize, Debug)]
pub struct GithubConfig {
    pub client_id: String,
    pub client_secret: String,
    pub resource_url: String,
    pub auth_url: String,
    pub token_url: String,
}

#[derive(Deserialize, Debug)]
pub struct SecurityConfig {
    pub jwt: JwtSecurityConfig,
    pub session: SessionSecurityConfig,
    pub rate_limiting: RateLimitingConfig,
    pub cors: CorsConfig,
    pub security_headers: SecurityHeadersConfig,
}

#[derive(Deserialize, Debug)]
pub struct JwtSecurityConfig {
    pub access_token_ttl: u64,
    pub refresh_token_ttl: u64,
    pub key_rotation_interval: u64,
    pub algorithm: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SessionSecurityConfig {
    pub cookie_ttl: u64,
    pub cache_ttl: u64,
    pub secure_cookies: bool,
    pub same_site: String,
    pub http_only: bool,
}

#[derive(Deserialize, Debug)]
pub struct RateLimitingConfig {
    pub enabled: bool,
    pub requests_per_minute: u32,
    pub burst_size: u32,
    pub cleanup_interval: u64,
}

#[derive(Deserialize, Debug)]
pub struct CorsConfig {
    pub enabled: bool,
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
    pub max_age: u64,
}

#[derive(Deserialize, Debug)]
pub struct SecurityHeadersConfig {
    pub hsts_enabled: bool,
    pub hsts_max_age: u64,
    pub csp_enabled: bool,
    pub csp_policy: String,
    pub x_frame_options: String,
    pub x_content_type_options: bool,
}
