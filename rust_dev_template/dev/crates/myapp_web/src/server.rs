use core::num::NonZeroU32;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use axum::{
    http::{HeaderName, Request, Response, StatusCode},
    Router,
};
use axum_governor::{extractor::PeerIp, nz, GovernorConfigBuilder, GovernorLayer, Quota};
use tokio::{net::TcpListener, signal};
use tower::ServiceBuilder;
use tower_http::{
    request_id::{MakeRequestUuid, SetRequestIdLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::{info, info_span, Span};

use crate::db::Db;
use crate::WEB_SERVER_LISTEN_ADDR;

use super::route;
use super::state::{Status, WebState};

const HEADER_NAME_REQUEST_ID: &str = "x-request-id";

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub rps: u32,
    pub timeout_secs: u64,
}

pub async fn run(config: ServerConfig, db: Db) -> Result<(), Box<dyn std::error::Error>> {
    let state = Arc::new(WebState::new(db));

    let router = Router::new()
        .merge(route::router())
        .with_state(state.clone())
        .layer(
            ServiceBuilder::new()
                .layer(SetRequestIdLayer::new(
                    HeaderName::from_static(HEADER_NAME_REQUEST_ID),
                    MakeRequestUuid,
                ))
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(|request: &Request<_>| {
                            let request_id = request
                                .headers()
                                .get(HEADER_NAME_REQUEST_ID)
                                .and_then(|v| v.to_str().ok())
                                .unwrap_or("-");
                            info_span!(
                                "http_request",
                                request_id = request_id,
                                http.method = %request.method(),
                                http.path = %request.uri().path(),
                                http.status = tracing::field::Empty,
                                latency_ms = tracing::field::Empty,
                            )
                        })
                        .on_response(|response: &Response<_>, latency: Duration, span: &Span| {
                            let status = response.status().as_u16();
                            span.record("http.status", status);
                            span.record("latency_ms", latency.as_millis() as u64);
                        }),
                )
                .layer(GovernorLayer::new(
                    GovernorConfigBuilder::default()
                        .with_extractor(PeerIp::default())
                        .expect_connect_info()
                        .quota_default(Quota::requests_per_second(
                            NonZeroU32::new(config.rps).unwrap_or_else(|| nz!(100u32)),
                        ))
                        .finish()?,
                ))
                .layer(TimeoutLayer::with_status_code(
                    StatusCode::REQUEST_TIMEOUT,
                    Duration::from_secs(config.timeout_secs),
                )),
        );

    state.set_status(Status::Running);
    let listener = TcpListener::bind(WEB_SERVER_LISTEN_ADDR).await?;
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal(state.clone()))
    .await?;

    Ok(())
}

async fn shutdown_signal(state: Arc<WebState>) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("install Ctrl+C handler must be successfully");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("install terminate signal handler must be successfully")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!(
        event = "signal_shutdown",
        "signal received, starting graceful shutdown"
    );
    state.set_status(Status::Draining);
    tokio::time::sleep(Duration::from_secs(5)).await; // wait for a while to let in-flight requests finish
    state.set_status(Status::Stopped);
}
