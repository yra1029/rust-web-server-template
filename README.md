# rust-web-server-template

A Rust web server template implementing **Hexagonal Architecture** with clean separation of concerns, following domain-driven design principles.

## Inspired By

This template draws inspiration from the following github repositories and common practices for Hexagonal Architecture and Domain-Driven Design

- [1](https://github.com/howtocodeit/hexarch/tree/3-simple-service) - 
- [2](https://github.com/softwaremill/rust-axum-sqlx-redis-ws-template/tree/main) - 
- [3](https://github.com/Thodin/axum-clean-architecture/tree/main) - 

## Description

This template provides a structured foundation for building scalable, maintainable Rust web services using:

- **Axum** - Modern, ergonomic web framework for Rust
- **SQLx** - Async PostgreSQL driver with compile-time query verification
- **Hexagonal Architecture** - Clean architecture pattern separating business logic from infrastructure
- **Repository Pattern** - Abstraction layer for data persistence
- **Domain-Driven Design** - Clear domain boundaries and business logic encapsulation

### Key Features

- **Layered Architecture**: Clear separation between presentation, application, domain, and infrastructure layers
- **Type-Safe Database Access**: SQLx with compile-time query verification
- **Async/Await**: Full async support using Tokio runtime
- **Error Handling**: Structured error handling with domain-specific error types
- **Request Tracing**: Built-in request logging and tracing support
- **Dependency Injection**: Clean dependency management through ports and adapters
- **Testable Design**: Easy to mock and test with trait-based abstractions
- **RESTful API**: Standard HTTP endpoints for CRUD operations

### Architecture Overview

The project follows **Hexagonal Architecture** principles:

- **Presentation Layer**: HTTP handlers, request/response mapping, API contracts
- **Application Layer**: Use case orchestration, service implementations
- **Domain Layer**: Business models, repository ports (interfaces), domain errors
- **Infrastructure Layer**: Database adapters, external service clients, configuration

This architecture ensures that business logic remains independent of infrastructure details, making the codebase:
- **Testable**: Easy to mock dependencies
- **Maintainable**: Clear boundaries and responsibilities
- **Flexible**: Can swap implementations without changing business logic
- **Scalable**: Well-organized structure for growing applications


### Project structure Structure
```
/src
  /presentation    # Axum routes, middleware, request/response mappers
    /handlers      # HTTP request handlers, API DTOs, error mapping
    /http.rs       # HTTP server setup, routing, AppState
  /application     # Service traits/implementations, DTOs, application errors
    /dto
    /flows         # Use case implementations (services)
      /user_service.rs  # UserService orchestrates user use cases
  /domain          # Models, repository traits (ports), domain-specific errors
    /user
      /model.rs    # User, CreateUser, UpdateUser domain models
      /repository.rs  # UserRepositoryPort (port/interface definition)
      /error.rs    # UserDomainError domain errors
  /infrastructure  # DB, metrics, RPC, repository implementations (adapters), config
    /storage
      /adapter     # Repository implementations (adapters)
        /user_repository.rs  # UserRepository implements UserRepositoryPort
      /postgres.rs # Database connection setup
    /config.rs     # Configuration management
```

## Storage Layer Architecture

This project follows **Hexagonal Architecture** (also known as Ports and Adapters) with the **Repository Pattern** for data persistence. The architecture separates concerns and allows for easy testing and swapping of storage implementations.

### Data Flow Through Layers

The data flows from the presentation layer down to storage through the following layers:

```
HTTP Request
    ↓
[Presentation Layer]
    - Handlers receive HTTP requests
    - Map HTTP DTOs to domain models
    - Handle HTTP-specific concerns (status codes, serialization)
    ↓
[Application Layer]
    - Services orchestrate use cases
    - Coordinate between domain logic and infrastructure
    - Handle application-level concerns
    ↓
[Domain Layer]
    - Domain models (User, CreateUser, UpdateUser)
    - Repository ports (interfaces/contracts)
    - Domain errors (UserDomainError)
    ↓
[Infrastructure Layer]
    - Repository adapters (concrete implementations)
    - Database connections (PostgreSQL via SQLx)
    - SQL queries and data mapping
    ↓
Database (PostgreSQL)
```

### Example: Creating a User

1. **Presentation Layer** (`user_handlers.rs`):
   - Receives `CreateUserRequestBody` (HTTP DTO)
   - Converts to domain `CreateUser` model
   - Calls `UserService::create_user()`

2. **Application Layer** (`user_service.rs`):
   - `UserService` receives `CreateUser`
   - Delegates to `UserRepositoryPort::create_user()`
   - Returns domain `User` model

3. **Domain Layer** (`repository.rs`):
   - Defines `UserRepositoryPort` trait (port/interface)
   - Specifies contract: `async fn create_user(CreateUser) -> Result<User, UserDomainError>`

4. **Infrastructure Layer** (`user_repository.rs`):
   - `UserRepository` implements `UserRepositoryPort` (adapter)
   - Executes SQL query using SQLx
   - Maps database rows to domain `User` model
   - Returns domain model or domain error

### Repository Pattern

The **Repository Pattern** abstracts data access logic and provides a collection-like interface for domain objects. In this project:

#### Ports (Interfaces) - Defined in Domain Layer

**Location**: `src/lib/domain/user/repository.rs`

Ports define the contract for data access operations without implementation details:

```rust
#[async_trait]
pub trait UserRepositoryPort {
    async fn create_user(&self, user: CreateUser) -> Result<User, UserDomainError>;
    async fn get_user(&self, id: String) -> Result<User, UserDomainError>;
    async fn update_user(&self, user: UpdateUser) -> Result<User, UserDomainError>;
    async fn delete_user(&self, id: String) -> Result<(), UserDomainError>;
}
```

**Characteristics**:
- Defined in the **domain layer** (business logic doesn't depend on infrastructure)
- Uses domain models (`User`, `CreateUser`, `UpdateUser`)
- Returns domain errors (`UserDomainError`)
- No knowledge of database technology (SQL, NoSQL, etc.)

#### Adapters (Implementations) - Defined in Infrastructure Layer

**Location**: `src/lib/infra/storage/adapter/user_repository.rs`

Adapters provide concrete implementations of ports:

```rust
#[async_trait]
impl UserRepositoryPort for UserRepository {
    async fn create_user(&self, user: CreateUser) -> Result<User, UserDomainError> {
        // SQLx implementation using PostgreSQL
        sqlx::query("INSERT INTO users ...")
            .execute(&*self.db)
            .await?;
        // ...
    }
}
```

**Characteristics**:
- Defined in the **infrastructure layer**
- Implements the port interface
- Contains database-specific code (SQL queries, connection handling)
- Maps between database representations and domain models
- Can be swapped without changing domain/application code

### Dependency Injection

The dependency flow follows the **Dependency Inversion Principle**:

```
Application Layer (depends on) → Domain Port (interface)
                                        ↑
Infrastructure Layer (implements) → Domain Port
```

- **Application layer** depends on **domain ports** (interfaces), not implementations
- **Infrastructure layer** implements **domain ports**
- This allows swapping implementations (e.g., PostgreSQL → MongoDB) without changing business logic, implementation of new repositories will be required.

### Repository Creation

**Location**: `src/lib/infra/storage/adapter/mod.rs`

Repositories are created using a factory function:

```rust
pub fn create_repositories(db: Db) -> eyre::Result<Repositories<UserRepository>> {
    Ok(Repositories {
        user_repository: UserRepository::new(db),
    })
}
```

This centralizes repository instantiation and makes dependency injection explicit.

### Pros and Cons

#### Pros

1. **Testability**: Easy to create mock implementations of ports for unit testing
   ```rust
   struct MockUserRepository;
   impl UserRepositoryPort for MockUserRepository { /* ... */ }
   ```

2. **Flexibility**: Can swap storage implementations without changing business logic
   - Switch from PostgreSQL to MongoDB
   - Use in-memory storage for testing
   - Add caching layer transparently

3. **Separation of Concerns**: Clear boundaries between layers
   - Domain logic is independent of infrastructure
   - Business rules don't depend on database details

4. **Maintainability**: Changes to database schema or queries are isolated to adapters
   - SQL changes don't affect domain models
   - Domain model changes are reflected in ports, making impact clear

5. **Technology Independence**: Domain and application layers don't know about SQLx, PostgreSQL, etc.

#### Cons

1. **Complexity**: More layers and abstractions than a simple CRUD app
   - Requires understanding of hexagonal architecture
   - More files and indirection

2. **Boilerplate**: Need to define ports and implement adapters
   - More code for simple operations
   - Type conversions between layers

3. **Learning Curve**: Developers need to understand the architecture
   - Where to add new features (which layer?)
   - How data flows through layers

4. **Performance Overhead**: Additional abstraction layers
   - Trait object dispatch (minimal in practice)
   - Multiple conversions (HTTP DTO → APP DTO →  Domain)

### Design Decision: Repository Abstraction Over Database Type

Initially, it was considered making repositories generic over the database type, for example:

```rust
pub trait UserRepositoryPort<T: DB> {
    async fn create_user(&self, user: CreateUser) -> Result<User, UserDomainError>;
    // ...
}
```

This approach would have provided:
- **Isolation of repository logic** from specific database implementations
- **Easier unit testing** with mock database types
- **Type-level guarantees** about database compatibility

However, this design was **intentionally skipped** for the following reasons:

#### Why It Was Rejected

1. **Unnecessary Complexity**: Adding generics over database types introduces significant complexity without proportional benefits
   - Additional Highly Generic and complex layer of abstraction which should isolate the particular DB interaction
   - More complex type signatures throughout the codebase

2. **Mock Testing Anti-Pattern**: While this would enable easier unit testing with mocks, it leads to **testing for testing's sake**:
   - Unit tests would only verify that mock methods were called
   - This doesn't validate actual database interactions or SQL correctness
   - Mocks can drift from real implementations, giving false confidence

3. **Better Testing Approach**: The recommended approach is **integration testing** with real database interactions:
   - Use [testcontainers](https://github.com/testcontainers/testcontainers-rs) to spin up real PostgreSQL instances
   - Test actual SQL queries and database behavior
   - Catch real issues like constraint violations, transaction problems, and query bugs
   - Validate the entire data flow from handler to database
   - Compile time schema and query check with SQLX macroses

#### Current Approach

The current implementation uses concrete database within repository and binds it to the SQLX library because:

- **Simplicity**: Clear, straightforward code that's easy to understand
- **Compile time guarantee**: SQLX macroses check schema and queries during the compile time
- **Real Testing**: Integration tests with testcontainers provide meaningful validation
- **Practical**: Most applications use a single database type, making generics unnecessary
- **Maintainability**: Less abstraction means easier debugging and maintenance
- **Industry Practice**: The GitHub repositories referenced in the "Inspired By" section also use concrete database types rather than generic abstractions over DB types, validating this pragmatic approach.

If the support for multiple database backends will be needed, separate repositories implementations should be considered:
- Separate repository implementations per database (e.g., `PostgresUserRepository`, `MongoUserRepository`)
- Factory pattern to select the appropriate implementation at runtime
- This maintains simplicity while providing flexibility when actually needed


