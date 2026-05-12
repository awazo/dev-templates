use std::process::ExitCode;

use tracing::{error, info};

use crate::config::Config;
use crate::server;
use crate::{db, query};

pub async fn run(config: Config, db: db::Db) -> ExitCode {
    // TODO: CHANGE HERE
    // TODO: if use db only, implement like this
    db.get_metrics().log_info();
    query::users::User::select_all(&db)
        .await
        .into_iter()
        .for_each(|user| info!(?user, "user found"));

    // TODO: if use web server, implement like this
    match server::run(server::ServerConfig::from(&config), db).await {
        Ok(_) => info!("web server exited gracefully"),
        Err(e) => {
            error!("web server exited with error: {e}");
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}
