# Global Git Contribution Graph – Rust API

Stateless aggregation engine for the **Global Git Contribution Graph** ecosystem.

This service is responsible for securely retrieving, aggregating, and normalizing contribution data from multiple Git providers (GitHub, GitLab, Forgejo, and custom providers).

> ⚠️ This project is currently under active development. APIs and internal structures may change.

---

## Overview

The `global_git_contribution_graph_api` is a **stateless Rust service** that:

* Receives user configuration
* Resolves user config via Redis (QR sync flow)
* Requests data from Git providers
* Aggregates contribution statistics
* Returns normalized results to Web and Mobile clients

It is designed to be:

* Stateless
* Secure
* Cache-enabled
* Provider-agnostic
* Horizontally scalable

---

## Architecture Role

Within the global system:

```
WebApp (Next.js)
        │
        ▼
Next.js API (Auth + Config Encryption)
        │
        ▼
Rust API (THIS SERVICE)
        │
        ├── Redis (temporary config storage)
        └── Git Providers APIs
```

### Key Responsibilities

* Decrypt user configuration (when required)
* Retrieve temporary configuration by ID (Redis)
* Call Git provider APIs
* Aggregate contribution data
* Normalize responses
* Return structured stats payload

---

## Core Features

### Stateless Aggregation Engine

The service does not store persistent user data.
All required configuration is:

* Sent user config
* Or resolved temporarily from Redis

---

### Multi-Provider Support

Supported providers (in progress):

* GitHub (public instance only)
* GitLab (public instance & user's self-hosted instance)
* Forgejo (user's self-hosted instance)
* More to come

The architecture allows easy addition of new providers.

---

### Redis Integration

Redis is used for:

* Temporary configuration storage
* QR code synchronization flow
* TTL-based config expiration
* Reducing load on the main database

The Rust API:

* Retrieves configuration by temporary ID
* Uses it to fetch provider data
* Does not persist it

---

### Secure Configuration Handling

Configurations may be:

* Encrypted before reaching this service
* Decrypted using a private key
* Retrieved via secure HTTPS calls

Sensitive tokens are never stored persistently by this API.

---

## Data Flow

### Standard Web Flow

1. User logs in (Next.js API)
2. WebApp requests stats (via API)
3. API validates user
4. Rust API fetches provider data
5. Data aggregated
6. Response returned
7. UI renders graph

---

### QR Code / Mobile Flow

1. Mobile scans QR
2. Retrieves temporary config ID
3. Calls Next API
4. Retrieves user config
5. Calls Rust API
6. Rust fetches provider data
7. Stats returned to mobile

---

## API Behavior

The API:

* Accepts HTTPS requests only
* Validates input payload
* Check Redis cache
* Fetches provider data using async HTTP requests
* Aggregates stats
* Returns structured JSON

Example response structure (simplified):

```json
{
  "user": "username",
  "providers": ["github", "gitlab"],
  "total_contributions": 1234,
  "daily_breakdown": [...],
  "repositories": [...]
}
```

> ⚠️ Response schema may evolve.

---

## Project Structure (High-Level)

Typical Rust service structure:

```
src/
 ├── main.rs
 ├── services.rs
 ├── state.rs
 ├── providers/
 └── graphql/
```

### Main Modules

* **main.rs** – Entry point
* **services.rs** – Aggregation logic
* **state.rs** – Shared structure containing Redis clients and Git provider clients 
* **providers/** – Git provider integrations
* **graphql/** – Data structures and resolvers

---

## Installation

### Prerequisites

* Rust
* Cargo
* Redis
* Git provider API credentials

---

### Clone Repository

```bash
git clone https://github.com/Global-Git-Contribution-Graph/global_git_contribution_graph_api.git
cd global_git_contribution_graph_api
```

---

### Build

```bash
cargo build --release
```

---

### Run

```bash
cargo run
```

Or in production:

```bash
cargo run --release
```

---

## Configuration

Environment variables typically required:

| Variable      | Description                       |
| ------------- | --------------------------------- |
| `REDIS_URL`   | Redis connection string           |
| `PRIVATE_KEY` | Private key for config decryption |
| `SERVER_PORT` | API listening port                |

> Exact variables may evolve during development.

---

## Development

### Run in Development Mode

```bash
cargo watch -x run
```

### Run Tests

```bash
cargo test
```

---

## Performance & Scalability

* Fully async (Tokio-based)
* Stateless design
* Horizontal scaling ready
* Redis-backed temporary config resolution

---

## Security Considerations

* No persistent storage of user tokens
* HTTPS-only communication
* Encrypted config handling
* Temporary ID-based access with TTL
* Provider tokens never logged

---

## Current Status

🚧 In Active Development

Planned improvements:

* Extended provider support
* Improved aggregation performance
* Better error normalization
* Rate-limit handling
* Metrics & observability integration
* OpenAPI documentation

Breaking changes may occur until first stable release.

---

## Contributing

Contributions are welcome.

Please:

1. Open an issue to discuss changes
2. Create a feature branch
3. Submit a pull request

---

## Related Projects

* Web Application + API Gateway (Next.js)
* Mobile Applications
* HTML Integration Widgets

All repositories are part of:

👉 [https://github.com/Global-Git-Contribution-Graph](https://github.com/Global-Git-Contribution-Graph)
