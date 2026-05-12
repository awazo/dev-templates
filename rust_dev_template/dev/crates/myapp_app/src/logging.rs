use tracing_subscriber::{
    filter::LevelFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

pub fn init() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(LevelFilter::INFO.to_string()));

    tracing_subscriber::registry()
        .with(filter)
        .with(
            fmt::layer()
                .json()
                .with_timer(fmt::time::UtcTime::rfc_3339())
                .with_span_events(fmt::format::FmtSpan::CLOSE)
                .with_current_span(true)
                .with_span_list(true)
                .with_writer(std::io::stdout),
        )
        .init();
}
