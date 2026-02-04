use std::sync::Arc;

use eyre::Context;
use axum::Router;
use axum::routing::{delete, get, post, put};
use serde::Serialize;
use tokio::net;

use crate::application::flows::user_service::UserServiceTrait;
use crate::presentation::handlers::user_handlers;

/// Generic response structure shared by all API responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResponseBody<T: Serialize> {
    status_code: u16,
    data: T,
}

/// The response data format for all error responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ErrorResponseData {
    pub message: String,
}

/// Configuration for the HTTP server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpServerConfig<'a> {
    pub port: &'a str,
}

#[derive(Clone)]
/// The global application state shared between all request handlers.
pub struct AppState {
    pub user_service: Arc<dyn UserServiceTrait + Send + Sync + 'static>,
}

/// The application's HTTP server. The underlying HTTP package is opaque to module consumers.
pub struct HttpServer {
    router: axum::Router,
    listener: net::TcpListener,
}

impl HttpServer {
    /// Returns a new HTTP server bound to the port specified in `config`.
    pub async fn new(
        user_service: Arc<dyn UserServiceTrait + Send + Sync + 'static>,
        config: HttpServerConfig<'_>,
    ) -> eyre::Result<Self> {
        let trace_layer = tower_http::trace::TraceLayer::new_for_http().make_span_with(
            |request: &axum::extract::Request<_>| {
                let uri = request.uri().to_string();
                tracing::info_span!("http_request", method = ?request.method(), uri)
            },
        );

        // Construct dependencies to inject into handlers.
        let state = AppState {
            user_service: user_service,
        };

        let router = axum::Router::new()
            .nest("/api", api_routes())
            .layer(trace_layer)
            .with_state(state);

        let listener = net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
            .await
            .with_context(|| format!("failed to listen on {}", config.port))?;

        Ok(Self { router, listener })
    }

    /// Runs the HTTP server.
    pub async fn run(self) -> eyre::Result<()> {
        tracing::debug!("listening on {}", self.listener.local_addr().unwrap());
        axum::serve(self.listener, self.router)
            .await
            .context("received error from running server")?;
        Ok(())
    }
}

fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/users", post(user_handlers::create_user))
        .route("/users/:id", get(user_handlers::get_user))
        .route("/users/:id", put(user_handlers::update_user))
        .route("/users/:id", delete(user_handlers::delete_user))
}