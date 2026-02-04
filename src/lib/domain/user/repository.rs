use async_trait::async_trait;
use crate::domain::user::{error::UserDomainError, model::{CreateUser, UpdateUser, User}};

/// Repository port (interface) for user data access operations.
///
/// This trait defines the contract for persisting and retrieving user data without
/// specifying implementation details. It follows the Repository Pattern and
/// Hexagonal Architecture principles, acting as a port that can be implemented
/// by various adapters (e.g., PostgreSQL, MongoDB, in-memory storage).
#[async_trait]
pub trait UserRepositoryPort {
    /// Creates a new user in the repository.
    async fn create_user(&self, user: CreateUser) -> Result<User, UserDomainError>;

    /// Retrieves a user by their unique identifier.
    async fn get_user(&self, id: String) -> Result<User, UserDomainError>;

    /// Updates an existing user in the repository.
    async fn update_user(&self, user: UpdateUser) -> Result<User, UserDomainError>;

    /// Deletes a user from the repository.
    async fn delete_user(&self, id: String) -> Result<(), UserDomainError>;
}