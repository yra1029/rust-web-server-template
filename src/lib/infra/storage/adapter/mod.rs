use crate::{domain::user::repository::UserRepositoryPort};


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

/// Factory function for creating repository instances.
///
/// This function initializes repository adapters using a provided creator function(later could be extended to create multiple repositories).
/// It centralizes repositories creation and makes dependency injection explicit at
/// application startup. The generic design allows for different database types and
/// repository implementations.
pub fn create_repositories<DB: Clone, UR, URC>(db: DB, repository_creator: URC) -> eyre::Result<Repositories<UR>>
where
    UR: UserRepositoryPort + Send + Sync + 'static,
    URC: FnOnce(DB) -> eyre::Result<UR>,
{
    let user_repository = repository_creator(db.clone())?;
    Ok(Repositories { user_repository })
}