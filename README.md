# Global Git Contribution Graph API (Rust)

A lightweight **Rust** backend exposing a **GraphQL API** to fetch and normalize contribution/activity data from multiple git forges (currently **GitHub** and **GitLab**). It runs an **Axum** web server with **GraphiQL** enabled for easy exploration.

> Repo: `Global-Git-Contribution-Graph/global_git_contribution_graph_api`

---

## Table of Contents

- [What it does](#what-it-does)
- [How it works](#how-it-works)
- [API](#api)
  - [Endpoints](#endpoints)
  - [GraphQL schema](#graphql-schema)
  - [Example queries](#example-queries)
- [Installation](#installation)
- [Running locally](#running-locally)
- [Configuration](#configuration)
- [Dependencies](#dependencies)
- [Troubleshooting](#troubleshooting)
- [Contributing](#contributing)
- [License](#license)

---

## What it does

This service exposes a single GraphQL query (`stats`) that returns a list of **daily contributions** (`date`, `contributionCount`) for a given provider:

- **GitHub**: Uses the GitHub **GraphQL API** to fetch the contributions calendar (weeks/days) and returns non-zero days.
- **GitLab**: Uses the GitLab REST API to fetch “pushed” events, aggregates commit counts per day, and returns a sorted history (by date).

---

## How it works

- Providers implement a shared `GitProvider` trait:
  - `get_name() -> String`
  - `get_stats(username, token, url) -> Result<Vec<(String, i64)>, String>` 
- The server registers both providers (`GitHub`, `GitLab`) in shared state and injects it into the GraphQL schema.
- Axum serves:
  - `POST /graphql` for GraphQL requests
  - `GET /` for GraphiQL UI

---

## API

### Endpoints

- `GET /` – GraphiQL UI 
- `POST /graphql` – GraphQL endpoint

Server binds to `0.0.0.0:3000` by default.

### GraphQL schema

The core query is:

- `stats(name: String!, username: String!, token: String!, url: String): Stats`

Where:

- `name`: provider name (e.g. `"GitHub"` or `"GitLab"`) (matched case-insensitively)
- `username`: username used in the path to the current git forge endpoint
- `token`:
  - Private Access Token: **PRIVATE-TOKEN** header
- `url`: required for self-hosted Git instances, ignored for GitHub

The response includes:

- `Stats { history: [DailyContribution!]! }`
- `DailyContribution { date: String!, contributionCount: Int! }`

### Example queries

#### GitHub

```graphql
query {
  stats(
    name: "GitHub"
    username: "YOUR_GITHUB_USERNAME"
    token: "YOUR_GITHUB_TOKEN"
  ) {
    history {
      date
      contributionCount
    }
  }
}
````

#### GitLab

```graphql
query {
  stats(
    name: "GitLab"
    username: "YOUR_GITLAB_USERNAME"
    token: "YOUR_GITLAB_TOKEN"
    url: "https://gitlab.example.com"
  ) {
    history {
      date
      contributionCount
    }
  }
}
```

---

## Installation

### Prerequisites

* Rust toolchain (supports **edition 2024**) ([GitHub][1]) – or use the provided Dev Container
* Cargo

Clone the repository:

```bash
git clone https://github.com/Global-Git-Contribution-Graph/global_git_contribution_graph_api.git
cd global_git_contribution_graph_api
```

Build:

```bash
cargo build
```

---

## Running locally

Run the server:

```bash
cargo run
```

Then open:

* GraphiQL UI: `http://127.0.0.1:3000/` ([GitHub][2])
* GraphQL endpoint: `http://127.0.0.1:3000/graphql` ([GitHub][2])

> Note: the server listens on `0.0.0.0:3000` (useful for Docker/devcontainers). ([GitHub][2])

---

## Configuration

This project currently uses **code defaults** (no `.env` config in the repo at the time of writing):

* Port/interface: hard-coded to `0.0.0.0:3000` ([GitHub][2])
* Authentication: passed as GraphQL arguments (`token`, and optionally `url`) ([GitHub][3])

### Dev Container (optional)

A `.devcontainer` is provided with a Rust image and recommended VS Code extensions. ([GitHub][4])

---

## Dependencies

Key crates used:

* `axum` (web server)
* `async-graphql` + `async-graphql-axum` (GraphQL + Axum integration)
* `reqwest` (HTTP client)
* `tokio` (async runtime)
* `serde` / `serde_json`
* `chrono`
* `async-trait` ([GitHub][1])

---

## Troubleshooting

* **GitHub returns empty history**

  * Ensure your token has access to the GitHub GraphQL API. ([GitHub][5])
  * This provider intentionally **filters out zero-contribution days**. ([GitHub][5])

* **GitLab fails with “URL is required”**

  * Provide the `url` argument (base URL of your GitLab instance). ([GitHub][6])

* **Rate limits / auth errors**

  * GitHub and GitLab may rate limit API calls; use appropriate tokens and consider caching upstream if you deploy this publicly.

---

## Contributing

Contributions are welcome:

1. Fork the repo
2. Create a feature branch (`git checkout -b feat/my-feature`)
3. Commit changes
4. Open a Pull Request

Ideas:

* Add caching (Redis)
