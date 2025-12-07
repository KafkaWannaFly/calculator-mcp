use crate::app_config::AppConfig;
use axum::BoxError;
use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::{Router, routing::get};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower::buffer::BufferLayer;
use tower::limit::RateLimitLayer;
use tower::timeout::TimeoutLayer;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::cors::CorsLayer;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

pub struct HttpServer {
    config: Arc<AppConfig>,
}

impl HttpServer {
    pub fn new(config: Arc<AppConfig>) -> Self {
        HttpServer { config }
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        let app = Router::new().route("/health", get(health_check)).layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err: BoxError| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled error: {}", err),
                    )
                }))
                .layer(TimeoutLayer::new(Duration::from_secs(30)))
                .layer(BufferLayer::new(1024))
                .layer(RateLimitLayer::new(100, Duration::from_secs(1)))
                .layer(RequestBodyLimitLayer::new(4 * 1024 * 1024))
                .layer(CatchPanicLayer::new())
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive()),
        );

        let addr = SocketAddr::from(([0, 0, 0, 0], self.config.http_server.port));
        let listener = TcpListener::bind(&addr).await?;

        info!("Server running on http://{}", addr);

        axum::serve(listener, app).await?;
        Ok(())
    }
}

async fn health_check() -> &'static str {
    "OK"
}
