use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::user::{error::UserDomainError, model::{CreateUser, UpdateUser, User}, repository::UserRepositoryPort};

/// Service trait for user operations.
///
/// This trait defines the business logic interface for user management operations.
/// Implementations should handle domain logic and coordinate with repositories for data access and other dependencies(rpc, metrics) via clearly defined interfaces(ports)
#[async_trait]
pub trait UserServiceTrait {
    /// Creates a new user.
    async fn create_user(&self, user: CreateUser) -> Result<User, UserDomainError>;

    /// Retrieves a user by ID.
    async fn get_user(&self, id: String) -> Result<User, UserDomainError>;

    /// Updates an existing user.
    async fn update_user(&self, user: UpdateUser) -> Result<User, UserDomainError>;

    /// Deletes a user by ID.
    async fn delete_user(&self, id: String) -> Result<(), UserDomainError>;
}

/// Service implementation for user operations.
///
/// This service acts as an application layer between the presentation layer (handlers)
/// and the domain/infrastructure layer (ports/adapters). It coordinates user-related
/// business logic and delegates data access to the repository.
pub struct UserService {
    /// The user repository for data access operations.
    user_repository: Arc<dyn UserRepositoryPort + Send + Sync +'static>,

    // Note: Services can depend on multiple ports (repositories, external services, event publishers, etc.)
    // to orchestrate use cases. They coordinate between domain logic and infrastructure adapters via ports.
}

impl UserService {
    /// Creates a new `UserService` instance.
    pub fn new(user_repository: Arc<dyn UserRepositoryPort + Send + Sync +'static>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl UserServiceTrait for UserService {
    /// Creates a new user by delegating to the repository.
    async fn create_user(&self, user: CreateUser) -> Result<User, UserDomainError> {
        self.user_repository.create_user(user).await
    }
    
    /// Retrieves a user by ID by delegating to the repository.
    async fn get_user(&self, id: String) -> Result<User, UserDomainError> {
        self.user_repository.get_user(id).await
    }
    
    /// Updates an existing user by delegating to the repository.
    async fn update_user(&self, user: UpdateUser) -> Result<User, UserDomainError> {
        self.user_repository.update_user(user).await
    }
    
    /// Deletes a user by ID by delegating to the repository.
    async fn delete_user(&self, id: String) -> Result<(), UserDomainError> {
        self.user_repository.delete_user(id).await
    }
}