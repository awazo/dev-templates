use crate::db::DbConfig;
use crate::server::ServerConfig;

use super::Config;

impl From<&Config> for DbConfig {
    fn from(config: &Config) -> Self {
        Self {
            connection_string: format!(
                "postgres://{}:{}@{}:{}/{}",
                config.db_user,
                config.db_password.expose(),
                config.db_host,
                config.db_port,
                config.db_name
            ),
            pool_max_connections: config.db_pool_max_connections,
            pool_min_connections: config.db_pool_min_connections,
            pool_connect_timeout_secs: config.db_pool_connect_timeout_secs,
            pool_idle_timeout_secs: config.db_pool_idle_timeout_secs,
            pool_max_lifetime_secs: config.db_pool_max_lifetime_secs,
        }
    }
}

impl From<&Config> for ServerConfig {
    fn from(config: &Config) -> Self {
        Self {
            rps: config.web_server_rps,
            timeout_secs: config.web_server_timeout_secs,
        }
    }
}
