use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde_json::json;
use tracing::{info, warn};

use crate::api::ApiResponseWithoutBody;
use crate::state::WebState;

pub(crate) fn router() -> Router<Arc<WebState>> {
    Router::new().route("/health", get(health_handler))
}

async fn health_handler(State(state): State<Arc<WebState>>) -> impl IntoResponse {
    if state.is_shutting_down() {
        warn!(
            event = "health_check",
            "health check failed because server is shutting down"
        );
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!(ApiResponseWithoutBody::error_only_msg(
                "Service is shutting down"
            ))),
        );
    }

    state.db.get_metrics().log_info();

    if !state.db.is_healthy().await {
        warn!(
            event = "health_check",
            "health check failed because database is unhealthy"
        );
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!(ApiResponseWithoutBody::error_only_msg(
                "Database is unhealthy"
            ))),
        );
    }

    info!(event = "health_check", "health check passed");
    (
        StatusCode::OK,
        Json(json!(ApiResponseWithoutBody::ok_only_msg(
            "Service is healthy"
        ))),
    )
}
