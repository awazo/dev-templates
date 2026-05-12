use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

#[derive(Debug)]
pub struct DbConfig {
    pub connection_string: String,
    pub pool_max_connections: u32,
    pub pool_min_connections: u32,
    pub pool_connect_timeout_secs: u64,
    pub pool_idle_timeout_secs: u64,
    pub pool_max_lifetime_secs: u64,
}

#[derive(Debug, Clone)]
pub struct Db {
    pub pool: PgPool,
}

impl Db {
    pub async fn connect(config: DbConfig) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(config.pool_max_connections)
            .min_connections(config.pool_min_connections)
            .acquire_timeout(Duration::from_secs(config.pool_connect_timeout_secs))
            .idle_timeout(Duration::from_secs(config.pool_idle_timeout_secs))
            .max_lifetime(Duration::from_secs(config.pool_max_lifetime_secs))
            .connect(&config.connection_string)
            .await?;

        Ok(Self { pool })
    }

    pub fn get_metrics(&self) -> PoolMetrics {
        PoolMetrics::from(self)
    }

    pub async fn is_healthy(&self) -> bool {
        sqlx::query("SELECT 1").execute(&self.pool).await.is_ok()
    }

    pub async fn is_ready(&self) -> bool {
        self.pool.num_idle() > 0 && self.is_healthy().await
    }
}

#[derive(Debug)]
pub struct PoolMetrics {
    pub size: u32,
    pub num_idle: usize,
    pub num_active: usize,
    pub config_max: u32,
    pub utilization_pct: f64,
}

impl PoolMetrics {
    pub fn from(db: &Db) -> Self {
        let size = db.pool.size();
        let num_idle = db.pool.num_idle();
        let config_max = db.pool.options().get_max_connections();
        let utilization_pct = if config_max > 0 {
            (size as f64 / config_max as f64) * 100.0
        } else {
            0.0
        };
        Self {
            size,
            num_idle,
            num_active: size as usize - num_idle,
            config_max,
            utilization_pct,
        }
    }

    pub fn log_info(&self) {
        tracing::info!(
            size = self.size,
            num_idle = self.num_idle,
            num_active = self.num_active,
            config_max = self.config_max,
            utilization_pct = format!("{:.2}", self.utilization_pct),
            "db pool metrics"
        );
    }
}
