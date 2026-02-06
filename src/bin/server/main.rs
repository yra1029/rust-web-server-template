use std::sync::Arc;

use rust_web_server_lib::application::flows::user_service::UserService;
use rust_web_server_lib::infra::config::Config;
use rust_web_server_lib::infra::storage::adapter::postgres::{create_postgres_repositories, db_connect};
use rust_web_server_lib::presentation::http::{HttpServer, HttpServerConfig};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let config = Config::from_env()?;

    // Initialize tracing subscriber for request logging
    tracing_subscriber::fmt::init();

    // Connect to the database
    let db = db_connect(&config).await;

    // Create repositories
    let repositories = create_postgres_repositories(db)?;

    // Create user service with the repository
    let user_service = Arc::new(UserService::new(Arc::new(repositories.user_repository)));

    // Create HTTP server configuration
    let server_config = HttpServerConfig {
        port: &config.server_port,
    };

    // Create and run the HTTP server
    let http_server = HttpServer::new(user_service, server_config).await?;
    http_server.run().await
}
