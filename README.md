# bidmart-admin-be

A Rust Axum REST API with Modular Clean Architecture for BidMart admin services.
Queries from existing bidmart_auth and bidmart_core databases.

## Tech Stack

- **Language**: Rust
- **Web Framework**: Axum
- **Database**: PostgreSQL with SQLx (dual connection to bidmart_auth + bidmart_core)
- **Authentication**: Calls bidmart-auth-be microservice
- **Architecture**: Modular Clean Architecture

## Project Structure

```
src/
├── infrastructure/           # Global Technical & Configuration
│   ├── config/              # Environment Variables
│   ├── database/            # DB Connection Pools (dual)
│   ├── logger/              # Logger Implementation
│   └── filters/             # Global Exception Filters
│
├── modules/                 # Feature Modules (Vertical Slices)
│   └── admin/            # Admin Module
│       ├── application/    # Use Cases & DTOs
│       ├── domain/        # Entities, Errors, Repository Interfaces
│       └── infrastructure/ # Controllers & Repositories
│
└── shared/                 # Cross-cutting Components
    └── domain/             # Result Pattern, etc.
```

## Prerequisites

- Rust (latest stable)
- PostgreSQL (bidmart_auth and bidmart_core databases must exist)
- bidmart-auth-be and bidmart-core-be must be running

## Setup

### 1. Clone and Setup

```bash
cd bidmart-admin-be

# Copy environment template
cp .env.example .env
```

### 2. Configure Environment

Edit `.env` file:

```env
APP_SERVER_HOST=0.0.0.0
APP_SERVER_PORT=8082
APP_AUTH_DATABASE_URL=postgres://postgres:password@localhost:5432/bidmart_auth
APP_CORE_DATABASE_URL=postgres://postgres:password@localhost:5432/bidmart_core
APP_AUTH_BASE_URL=http://localhost:8080
```

### 3. Database Setup

This service does NOT create its own database. It connects to:
- `bidmart_auth` - for user data (admin = user with role=ADMIN)
- `bidmart_core` - for core business data (categories, listings, orders, etc.)

Ensure both databases exist and are populated by their respective services.

### 4. Run

```bash
# Development
cargo run
```

Server starts at `http://localhost:8082`

## API Endpoints

### Health Check

```bash
curl http://localhost:8082/health
```

**Response:**
```json
{
  "service": "bidmart-admin-be",
  "status": "ok"
}
```

### Ready Check

```bash
curl http://localhost:8082/ready
```

## Architecture Notes

This service queries from existing databases:

| Database | Tables Accessed | Purpose |
|----------|---------------|---------|
| bidmart_auth | users | Admin user lookup (role=ADMIN) |
| bidmart_core | categories, listings, bids, orders, wallets, etc. | Core business data |

**Admin users** are defined as users with `role=ADMIN` in the `bidmart_auth.users` table.

This service is extensible - add use cases, controllers, and repositories as needed for specific admin operations.

## Build

```bash
cargo build
```

## License

MIT
