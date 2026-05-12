use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde_json::json;
use tracing::{info, instrument};

use crate::api::ApiResponse;
use crate::query;
use crate::state::WebState;

pub(crate) fn router() -> Router<Arc<WebState>> {
    Router::new().route("/example", get(example_handler))
}

#[instrument(skip(state))]
async fn example_handler(State(state): State<Arc<WebState>>) -> impl IntoResponse {
    let users = query::users::User::select_all(&state.db).await;
    info!(count = users.len(), "fetched users from database");

    (StatusCode::OK, Json(json!(ApiResponse::ok(users))))
}
