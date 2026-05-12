pub mod example;
pub mod health;
pub mod ready;

use std::sync::Arc;

use axum::Router;

use crate::state::WebState;

pub(crate) fn router() -> Router<Arc<WebState>> {
    Router::new()
        .merge(health::router())
        .merge(ready::router())
        .merge(example::router())
}
