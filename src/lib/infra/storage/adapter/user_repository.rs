use async_trait::async_trait;
use sqlx::Row;
use uuid::Uuid;

use crate::domain::user::{error::UserDomainError, model::{CreateUser, UpdateUser, User}, repository::UserRepositoryPort};
use crate::infra::storage::postgres::Db;

/// PostgreSQL implementation of the user repository.
///
/// This repository provides data access operations for users using SQLx and PostgreSQL.
/// It implements the [`UserRepositoryTrait`] and handles all CRUD operations for the `users` table.
pub struct UserRepository {
    /// The PostgreSQL database connection pool.
    db: Db,
}

impl UserRepository {
    /// Creates a new `UserRepository` instance.
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepositoryPort for UserRepository {
    async fn create_user(&self, user: CreateUser) -> Result<User, UserDomainError> {
        let id = Uuid::new_v4().to_string();
        
        // Better to use sqlx::query! macro for compile-time verification of the schema and query. Used functions because of absence of installed locally db.
        sqlx::query(
            r#"
            INSERT INTO users (id, name, email, age)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(&id)
        .bind(&user.name)
        .bind(&user.email)
        .bind(user.age as i16)
        .execute(&*self.db)
        .await
        .map_err(|e| {
            if e.to_string().contains("duplicate") || e.to_string().contains("unique") {
                UserDomainError::UserAlreadyExists
            } else {
                tracing::error!("Failed to create user: {}", e);
                UserDomainError::UserCreationFailed
            }
        })?;

        Ok(User::new(id, user.name, user.email, user.age))
    }

    async fn get_user(&self, id: String) -> Result<User, UserDomainError> {
                // Better to use sqlx::query! macro for compile-time verification of the schema and query. Used functions because of absence of installed locally db.
        let row = sqlx::query(
            r#"
            SELECT id, name, email, age
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(&id)
        .fetch_optional(&*self.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get user: {}", e);
            UserDomainError::UserNotFound
        })?;

        match row {
            Some(row) => {
                let id: String = row.get("id");
                let name: String = row.get("name");
                let email: String = row.get("email");
                let age: i16 = row.get("age");
                Ok(User::new(id, name, email, age as u8))
            }
            None => Err(UserDomainError::UserNotFound),
        }
    }

    async fn update_user(&self, user: UpdateUser) -> Result<User, UserDomainError> {
        // First, get the existing user to merge with updates
        let existing = self.get_user(user.id.clone()).await?;

        let name = user.name.unwrap_or_else(|| existing.name().to_string());
        let email = user.email.unwrap_or_else(|| existing.email().to_string());
        let age = user.age.unwrap_or(existing.age());

                // Better to use sqlx::query! macro for compile-time verification of the schema and query. Used functions because of absence of installed locally db.
        sqlx::query(
            r#"
            UPDATE users
            SET name = $1, email = $2, age = $3, updated_at = CURRENT_TIMESTAMP
            WHERE id = $4
            "#,
        )
        .bind(&name)
        .bind(&email)
        .bind(age as i16)
        .bind(&user.id)
        .execute(&*self.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update user: {}", e);
            UserDomainError::UserUpdateFailed
        })?;

        Ok(User::new(user.id, name, email, age))
    }

    async fn delete_user(&self, id: String) -> Result<(), UserDomainError> {
        // Better to use sqlx::query! macro for compile-time verification of the schema and query. Used functions because of absence of installed locally db.
        let rows_affected = sqlx::query(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
        )
        .bind(&id)
        .execute(&*self.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete user: {}", e);
            UserDomainError::UserDeletionFailed
        })?
        .rows_affected();

        if rows_affected == 0 {
            Err(UserDomainError::UserNotFound)
        } else {
            Ok(())
        }
    }
}

