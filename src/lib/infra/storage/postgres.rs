use std::sync::Arc;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::infra::config::Config;


pub type Db = Arc<Pool<Postgres>>;

pub async fn db_connect(config: &Config) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(config.database_url.as_str())
        .await
        .expect("Error connecting to database")
}