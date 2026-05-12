use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde_json::json;
use tracing::{info, warn};

use crate::api::ApiResponseWithoutBody;
use crate::state::WebState;

pub(crate) fn router() -> Router<Arc<WebState>> {
    Router::new().route("/ready", get(ready_handler))
}

async fn ready_handler(State(state): State<Arc<WebState>>) -> impl IntoResponse {
    if !state.is_ready() {
        warn!(
            event = "ready_check",
            "ready check failed because server is not ready"
        );
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!(ApiResponseWithoutBody::error_only_msg(
                "Service is not ready"
            ))),
        );
    }

    state.db.get_metrics().log_info();

    if !state.db.is_ready().await {
        warn!(
            event = "ready_check",
            "ready check failed because database is not ready"
        );
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!(ApiResponseWithoutBody::error_only_msg(
                "Database is not ready"
            ))),
        );
    }

    info!(event = "ready_check", "ready check passed");
    (
        StatusCode::OK,
        Json(json!(ApiResponseWithoutBody::ok_only_msg(
            "Service is ready"
        ))),
    )
}
