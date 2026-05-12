pub mod convert;
mod secret;

use std::{env, fmt::Display, str::FromStr};

pub use secret::Secret;

#[derive(Debug)]
pub struct Config {
    pub app_env: String,
    pub db_user: String,
    pub db_password: Secret<String>,
    pub db_host: String,
    pub db_port: u16,
    pub db_name: String,
    pub db_pool_max_connections: u32,
    pub db_pool_min_connections: u32,
    pub db_pool_connect_timeout_secs: u64,
    pub db_pool_idle_timeout_secs: u64,
    pub db_pool_max_lifetime_secs: u64,
    pub web_server_rps: u32,
    pub web_server_timeout_secs: u64,
}

impl Config {
    pub fn from_env() -> Result<Self, LoadError> {
        Self::init();

        Ok(Self {
            app_env: load("APP_ENV").unwrap_or_else(|_| "dev".to_string()),
            db_user: load("DB_USER")?,
            db_password: Secret::new(load("DB_PASSWORD")?),
            db_host: load("DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
            db_port: load("DB_PORT").unwrap_or(5432),
            db_name: load("DB_NAME")?,
            db_pool_max_connections: load("DB_POOL_MAX_CONNECTIONS").unwrap_or(10),
            db_pool_min_connections: load("DB_POOL_MIN_CONNECTIONS").unwrap_or(1),
            db_pool_connect_timeout_secs: load("DB_POOL_CONNECT_TIMEOUT_SECS").unwrap_or(5),
            db_pool_idle_timeout_secs: load("DB_POOL_IDLE_TIMEOUT_SECS").unwrap_or(600),
            db_pool_max_lifetime_secs: load("DB_POOL_MAX_LIFETIME_SECS").unwrap_or(1800),
            web_server_rps: load("WEB_SERVER_RPS").unwrap_or(100),
            web_server_timeout_secs: load("WEB_SERVER_TIMEOUT_SECS").unwrap_or(30),
        })
    }

    #[cfg(test)]
    fn init() {
        dotenvy::from_filename(".env.test").ok();
    }
    #[cfg(not(test))]
    fn init() {
        let app_env = env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string());
        dotenvy::from_filename(format!(".env.{}", app_env)).ok();
    }

    pub fn is_dev(&self) -> bool {
        self.app_env == "dev"
    }

    pub fn is_test(&self) -> bool {
        self.app_env == "test"
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("env '{key}' missing: {source}")]
    Missing {
        key: String,
        #[source]
        source: env::VarError,
    },
    #[error("env '{key}' parse error: {message}")]
    Parse { key: String, message: String },
}

fn load<T>(key: &str) -> Result<T, LoadError>
where
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    let val = env::var(key).map_err(|e| LoadError::Missing {
        key: key.to_string(),
        source: e,
    })?;
    val.parse::<T>().map_err(|e| LoadError::Parse {
        key: key.to_string(),
        message: e.to_string(),
    })
}
