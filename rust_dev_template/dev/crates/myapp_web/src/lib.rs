mod api;
mod route;
pub mod server;
mod state;

use myapp_db::*;

const WEB_SERVER_LISTEN_ADDR: &str = "0.0.0.0:8080";
