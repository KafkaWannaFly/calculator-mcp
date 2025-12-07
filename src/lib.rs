use std::sync::Arc;

use tracing_subscriber::{EnvFilter, fmt::time::UtcTime};

use crate::{app_config::AppConfig, http_server::HttpServer};

pub mod app_config;
pub mod evaluator;
pub mod http_server;

pub fn init() -> anyhow::Result<HttpServer> {
    init_tracing();

    let app_config = Arc::new(AppConfig::new_from_file("config.toml")?);
    let http_server = HttpServer::new(app_config.clone());
    Ok(http_server)
}

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_timer(UtcTime::rfc_3339())
        .with_target(true)
        .with_level(true)
        .with_file(true)
        .with_line_number(true)
        .with_ansi(true)
        .init();
}
