mod cmd;
mod config;
mod logging;
mod run;

use std::process::ExitCode;

use tracing::{error, info};

use myapp_db::*;
use myapp_web::*;

#[tokio::main]
async fn main() -> ExitCode {
    logging::init();

    let config = match config::Config::from_env() {
        Ok(config) => config,
        Err(e) => {
            error!("failed to load config: {e}");
            return ExitCode::FAILURE;
        }
    };
    info!(?config, "config loaded");

    let db = match db::Db::connect(db::DbConfig::from(&config)).await {
        Ok(db) => db,
        Err(e) => {
            error!("failed to connect to db: {e}");
            return ExitCode::FAILURE;
        }
    };
    info!("db connected");

    match cmd::parse() {
        cmd::Cmd::Migrate => {
            if run_migrations(&db).await.is_none() {
                return ExitCode::FAILURE;
            }
            return ExitCode::SUCCESS;
        }
        cmd::Cmd::None => {} // continue with normal processing
    }

    if config.is_dev() {
        if run_migrations(&db).await.is_none() {
            return ExitCode::FAILURE;
        }
    }

    if run::run(config, db).await == ExitCode::FAILURE {
        return ExitCode::FAILURE;
    }

    return ExitCode::SUCCESS;
}

async fn run_migrations(db: &db::Db) -> Option<()> {
    sqlx::migrate!("./migrations")
        .run(&db.pool)
        .await
        .inspect_err(|e| error!("failed to run migrations: {e}"))
        .inspect(|_| info!("migrations ran successfully"))
        .ok()
}
