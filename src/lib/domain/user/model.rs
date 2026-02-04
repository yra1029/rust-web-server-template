/// Domain model representing a User entity.
///
/// This is the core domain entity that encapsulates user business logic and data.
pub struct User {
    id: String,
    name: String,
    email: String,
    age: u8,
}

impl User {
    /// Creates a new `User` instance.
    pub fn new(id: String, name: String, email: String, age: u8) -> Self {
        Self { id, name, email, age }
    }

    /// Returns the user's unique identifier.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the user's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the user's email address.
    pub fn email(&self) -> &str {
        &self.email
    }

    /// Returns the user's age.
    pub fn age(&self) -> u8 {
        self.age
    }
}

/// Data transfer object for creating a new user.
///
/// This struct represents the data required to create a user in the system.
pub struct CreateUser {
    /// The user's full name.
    pub name: String,
    /// The user's email address.
    pub email: String,
    /// The user's age.
    pub age: u8,
}

/// Data transfer object for updating an existing user.
///
/// This struct represents partial update data for a user. All fields are optional,
pub struct UpdateUser {
    /// The unique identifier of the user to update.
    pub id: String,
    /// Optional new name for the user. If `None`, the existing name is preserved.
    pub name: Option<String>,
    /// Optional new email for the user. If `None`, the existing email is preserved.
    pub email: Option<String>,
    /// Optional new age for the user. If `None`, the existing age is preserved.
    pub age: Option<u8>,
}