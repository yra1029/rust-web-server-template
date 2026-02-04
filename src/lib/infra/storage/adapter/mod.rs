use crate::{domain::user::repository::UserRepositoryPort, infra::storage::adapter::postgres::{ Db, user_repository::UserRepository}};


pub mod postgres;

/// Container for all repository implementations (adapters).
///
/// This struct groups all repository adapters together, making it easy to pass
/// them as a unit to services or other components. It uses generics to allow
/// for different repository implementations (e.g., PostgreSQL, MongoDB, in-memory)
/// while maintaining type safety.
pub struct Repositories<UR: UserRepositoryPort> where UR: Send + Sync + 'static {
    /// The user repository adapter implementation.
    pub user_repository: UR,
}

/// Factory function for creating concrete repository instances.
///
/// This function initializes all repository adapters with the provided database
/// connection. It centralizes repository creation and makes dependency injection
/// explicit at the application startup.
pub fn create_repositories(db: Db) -> eyre::Result<Repositories<UserRepository>> {
    Ok(Repositories {
        user_repository: UserRepository::new(db),
    })
}