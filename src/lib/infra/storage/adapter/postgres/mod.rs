pub mod user_repository;

use std::sync::Arc;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::infra::{config::Config, storage::{StorageRepositories, adapter::postgres::user_repository::UserRepository, create_repositories}};

pub type Db = Arc<Pool<Postgres>>;

pub async fn db_connect(config: &Config) -> Db {
    Arc::new(PgPoolOptions::new()
        .max_connections(5)
        .connect(config.database_url.as_str())
        .await
        .expect("Error connecting to database")
    )
}

pub fn create_postgres_repositories(db: Db) -> eyre::Result<StorageRepositories<UserRepository>> {
    create_repositories(db, |db| Ok(UserRepository::new(db)))
}