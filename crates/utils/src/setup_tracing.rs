use anyhow::Result;
use std::sync::OnceLock;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

static LOG_INIT: OnceLock<()> = OnceLock::new();

pub fn setup_logging() -> Result<()> {
    if LOG_INIT.set(()).is_err() {
        return Err(anyhow::anyhow!("Setup logging can only be ran once"));
    }
    let env_filter = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;

    let fmt_layer = fmt::layer()
        .with_target(false)
        .with_file(true)
        .with_line_number(true)
        .compact();

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();

    Ok(())
}
