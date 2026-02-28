# Clean Architecture Axum Project Setup Guide

This document provides instructions for agents to replicate the `bidmart-admin-be` project structure.

## Project Overview

- **Project Name**: bidmart-admin-be
- **Type**: Rust Axum REST API
- **Architecture**: Modular Clean Architecture
- **Database**: PostgreSQL with SQLx (dual connection to bidmart_auth + bidmart_core)
- **Authentication**: Calls bidmart-auth-be microservice

## Directory Structure

```
<project-name> /
├── Cargo.toml
├── build.rs
├── .env.example
├── migrations/                  # (placeholder - no local migrations)
└── src/
    ├── main.rs
    ├── lib.rs
    ├── infrastructure/           # Global Technical & Configuration
    │   ├── config/             # Environment Variables
    │   ├── database/           # DB Connection Pools (dual)
    │   ├── logger/             # Logger Implementation
    │   └── filters/            # Global Exception Filters
    │
    ├── modules/                 # Feature Modules (Vertical Slices)
    │   └── admin/            # Admin Module
    │       ├── application/   # Use Cases & DTOs
    │       │   ├── dto/
    │       │   └── use_cases/
    │       ├── domain/        # Entities, Errors, Repository Interfaces
    │       │   ├── entities/
    │       │   ├── errors/
    │       │   └── traits/
    │       └── infrastructure/ # Controllers, Repositories & Services
    │           ├── controllers/
    │           ├── repositories/
    │           └── services/
    │
    └── shared/                 # Cross-cutting Components
        └── domain/            # Result Pattern, etc.
```

## Configuration

### Environment Variables

Create `.env` file:

```env
APP_SERVER_HOST=0.0.0.0
APP_SERVER_PORT=8082
APP_AUTH_DATABASE_URL=postgres://postgres:password@localhost:5432/bidmart_auth
APP_CORE_DATABASE_URL=postgres://postgres:password@localhost:5432/bidmart_core
APP_AUTH_BASE_URL=http://localhost:8080
```

### Key Differences from bidmart-auth-be and bidmart-core-be

1. **Port**: 8082 (auth is 8080, core is 8081)
2. **Database**: Dual connection to bidmart_auth + bidmart_core (no own database)
3. **Auth**: Calls bidmart-auth-be microservice for token validation
4. **Admin Definition**: Admin = user with role=ADMIN in bidmart_auth.users table

## API Endpoints

- `GET /health` - Health check
- `GET /ready` - Readiness check

Additional endpoints will be added as admin operations are implemented.

## Running the Project

```bash
cargo build
cargo run
```

## Running Tests

```bash
cargo test
```

## Database Access

This service connects to two databases:

| Database | Purpose |
|----------|---------|
| bidmart_auth | Query users table for admin lookup |
| bidmart_core | Query core business data (categories, listings, orders, etc.) |

No local migrations - migrations are handled by bidmart-auth-be and bidmart-core-be.

## Extensibility

This service is designed to be extended with admin operations. Add:
- Use cases in `src/modules/admin/application/use_cases/`
- Controllers in `src/modules/admin/infrastructure/controllers/`
- Repositories in `src/modules/admin/infrastructure/repositories/`

Example operations to implement:
- List users from bidmart_auth
- Manage categories from bidmart_core
- Manage listings, orders, etc.
