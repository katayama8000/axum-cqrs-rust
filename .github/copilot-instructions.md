# GitHub Copilot Instructions

## Project Overview
This project is a circle management system built with Rust, implementing CQRS (Command Query Responsibility Segregation) architecture and Event Sourcing patterns.

## Technology Stack
- **Language**: Rust
- **Web Framework**: Axum
- **Database**: MariaDB
- **Cache**: Redis
- **Container**: Docker

## Architecture
- **CQRS**: Separates command and query responsibilities
- **Event Sourcing**: Persists events to reconstruct application state
- **Hexagonal Architecture**: Isolates domain logic from external dependencies

## Project Structure
```
src/
├── bin/main.rs              # Application entry point
├── crates/
│   ├── api/                 # Web API layer (Controllers, Routes)
│   ├── command/             # Command handlers (write operations)
│   ├── domain/              # Domain logic (business rules)
│   ├── infrastructure/      # Infrastructure layer (DB, Redis)
│   ├── main/                # Application configuration and startup
│   └── query/               # Query handlers (read operations)
```

## Coding Standards and Best Practices

### 1. Rust Coding Style
- Use standard Rust formatting following `rustfmt`
- Address `clippy` warnings
- Proper error handling (use of `Result<T, E>`)
- Appropriate use of `#[derive]` macros

### 2. Architecture Principles
- **Domain layer**: Contains only business logic, no external dependencies
- **Infrastructure layer**: Integration with external systems (DB, Redis)
- **API layer**: HTTP request/response handling
- **Command layer**: Processing write operations
- **Query layer**: Processing read operations

### 3. CQRS Implementation Patterns
- Clear separation between commands and queries
- Commands have side effects but minimal return values
- Queries have no side effects and return data
- Data persistence through event store

### 4. Error Handling
- Use `anyhow` crate for error chaining
- Appropriate error messages and logging
- Proper HTTP status code setting

### 5. Test Code
- Unit tests in each crate's `tests/` directory
- Mocking using `mockall` crate
- Integration tests for API endpoints

### 6. Database Operations
- SQL queries managed in `sql/` directory
- Proper naming of migration scripts
- Transaction management considerations

### 7. Asynchronous Programming
- Use `tokio` runtime
- Proper implementation of `async/await` patterns
- Avoid data races in concurrent processing

## Guidelines for New Feature Implementation

### When Adding Domain Entities
1. Define entities and value objects in `domain/` crate
2. Implement proper validation rules
3. Event definition and persistence functionality

### When Adding API Endpoints
1. Implement router and handlers in `api/` crate
2. Define request/response structures
3. Set appropriate HTTP status codes
4. Update OpenAPI specification (if used)

## Commonly Used Commands
```bash
# Start development server
cargo run

# Watch mode startup
./watch.sh

# Run tests
cargo test

# Code formatting
cargo fmt

# Linting
cargo clippy

# Build
cargo build --release
```

## Code Generation Guidelines
When generating code for this project, follow the above rules and patterns, maintaining consistency with existing code style. Particularly, adhere to CQRS architecture principles and ensure proper responsibility allocation to appropriate layers.

## Comements on Code
- Write a comment in English if the code is complex or non-obvious
- Use Rust doc comments (`///`) for public functions and structs
- You can write a comment in japanese to explain in markdown format if necessary

## You can not use docker-compose command
- You are in a devcontainer environment and you can not use docker-compose command