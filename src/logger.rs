use tracing_subscriber::{EnvFilter, FmtSubscriber};

/// Initializes the global logger for the application.
pub fn init() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info")); // Default to `info` level if not set

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(filter)
        .with_target(true) // Include module path in logs
        .with_timer(tracing_subscriber::fmt::time::UtcTime::rfc_3339())
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default subscriber failed");
}
