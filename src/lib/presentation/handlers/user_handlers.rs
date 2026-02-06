use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

use crate::domain::user::{error::UserDomainError, model::{CreateUser, UpdateUser, User}};
use crate::presentation::http::AppState;

#[derive(Debug, Clone)]
pub struct ApiSuccess<T: Serialize + PartialEq>(StatusCode, Json<ApiResponseBody<T>>);

impl<T> PartialEq for ApiSuccess<T>
where
    T: Serialize + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 .0 == other.1 .0
    }
}

impl<T: Serialize + PartialEq> ApiSuccess<T> {
    fn new(status: StatusCode, data: T) -> Self {
        ApiSuccess(status, Json(ApiResponseBody::new(status, data)))
    }
}

impl<T: Serialize + PartialEq> IntoResponse for ApiSuccess<T> {
    fn into_response(self) -> Response {
        (self.0, self.1).into_response()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiError {
    InternalServerError(String),
    UnprocessableEntity(String),
    NotFound(String),
}

impl From<UserDomainError> for ApiError {
    fn from(e: UserDomainError) -> Self {
        match e {
            UserDomainError::UserNotFound => {
                Self::NotFound("User not found".to_string())
            }
            UserDomainError::UserAlreadyExists => {
                Self::UnprocessableEntity("User already exists".to_string())
            }
            UserDomainError::UserCreationFailed => {
                Self::InternalServerError("Failed to create user".to_string())
            }
            UserDomainError::UserUpdateFailed => {
                Self::InternalServerError("Failed to update user".to_string())
            }
            UserDomainError::UserDeletionFailed => {
                Self::InternalServerError("Failed to delete user".to_string())
            }
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        use ApiError::*;

        match self {
            InternalServerError(e) => {
                tracing::error!("{}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponseBody::new_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal server error".to_string(),
                    )),
                )
                    .into_response()
            }
            UnprocessableEntity(message) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ApiResponseBody::new_error(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    message,
                )),
            )
                .into_response(),
            NotFound(message) => (
                StatusCode::NOT_FOUND,
                Json(ApiResponseBody::new_error(
                    StatusCode::NOT_FOUND,
                    message,
                )),
            )
                .into_response(),
        }
    }
}

/// Generic response structure shared by all API responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiResponseBody<T: Serialize + PartialEq> {
    status_code: u16,
    data: T,
}

impl<T: Serialize + PartialEq> ApiResponseBody<T> {
    pub fn new(status_code: StatusCode, data: T) -> Self {
        Self {
            status_code: status_code.as_u16(),
            data,
        }
    }
}

impl ApiResponseBody<ApiErrorData> {
    pub fn new_error(status_code: StatusCode, message: String) -> Self {
        Self {
            status_code: status_code.as_u16(),
            data: ApiErrorData { message },
        }
    }
}

/// The response data format for all error responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiErrorData {
    pub message: String,
}

/// The body of a User creation request.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CreateUserRequestBody {
    pub name: String,
    pub email: String,
    pub age: u8,
}

/// The response body data field for successful User creation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreateUserResponseData {
    pub id: String,
    pub name: String,
    pub email: String,
    pub age: u8,
}

/// The body of a User update request.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct UpdateUserRequestBody {
    pub name: Option<String>,
    pub email: Option<String>,
    pub age: Option<u8>,
}

/// The response body data field for successful User retrieval/update.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UserResponseData {
    pub id: String,
    pub name: String,
    pub email: String,
    pub age: u8,
}

impl From<&User> for CreateUserResponseData {
    fn from(user: &User) -> Self {
        Self {
            id: user.id().to_string(),
            name: user.name().to_string(),
            email: user.email().to_string(),
            age: user.age(),
        }
    }
}

impl From<(String, UpdateUserRequestBody)> for UpdateUser {
    fn from((id, body): (String, UpdateUserRequestBody)) -> Self {
        UpdateUser {
            id,
            name: body.name,
            email: body.email,
            age: body.age,
        }
    }
}

impl From<&User> for UserResponseData {
    fn from(user: &User) -> Self {
        Self {
            id: user.id().to_string(),
            name: user.name().to_string(),
            email: user.email().to_string(),
            age: user.age(),
        }
    }
}

/// Create a new User.
///
/// # Responses
///
/// - 201 Created: the User was successfully created.
/// - 422 Unprocessable entity: A User with the same email already exists.
/// - 500 Internal server error: Failed to create user.
pub async fn create_user(
    State(state): State<AppState>,
    Json(body): Json<CreateUserRequestBody>,
) -> Result<ApiSuccess<CreateUserResponseData>, ApiError> {
    let create_user = CreateUser {
        name: body.name,
        email: body.email,
        age: body.age,
    };

    state
        .user_service
        .create_user(create_user)
        .await
        .map_err(ApiError::from)
        .map(|user| ApiSuccess::new(StatusCode::CREATED, CreateUserResponseData::from(&user)))
}

/// Get a User by ID.
///
/// # Responses
///
/// - 200 OK: the User was found.
/// - 404 Not Found: the User was not found.
/// - 500 Internal server error: Failed to get user.
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<ApiSuccess<UserResponseData>, ApiError> {
    state
        .user_service
        .get_user(id)
        .await
        .map_err(ApiError::from)
        .map(|user| ApiSuccess::new(StatusCode::OK, UserResponseData::from(&user)))
}

/// Update a User.
///
/// # Responses
///
/// - 200 OK: the User was successfully updated.
/// - 404 Not Found: the User was not found.
/// - 500 Internal server error: Failed to update user.
pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<UpdateUserRequestBody>,
) -> Result<ApiSuccess<UserResponseData>, ApiError> {
    let update_user = UpdateUser::from((id, body));

    state
        .user_service
        .update_user(update_user)
        .await
        .map_err(ApiError::from)
        .map(|user| ApiSuccess::new(StatusCode::OK, UserResponseData::from(&user)))
}

/// Delete a User by ID.
///
/// # Responses
///
/// - 204 No Content: the User was successfully deleted.
/// - 404 Not Found: the User was not found.
/// - 500 Internal server error: Failed to delete user.
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    state
        .user_service
        .delete_user(id)
        .await
        .map_err(ApiError::from)
        .map(|_| StatusCode::NO_CONTENT)
}
