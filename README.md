# Global Git Contribution Graph API (Rust)

A lightweight **Rust** backend exposing a **GraphQL API** to fetch and normalize contribution/activity data from multiple git forges (currently **GitHub** and **GitLab**). It runs an **Axum** web server with **GraphiQL** enabled for easy exploration.

> Repo: `Global-Git-Contribution-Graph/global_git_contribution_graph_api` :contentReference[oaicite:0]{index=0}

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

- **GitHub**: Uses the GitHub **GraphQL API** to fetch the contributions calendar (weeks/days) and returns non-zero days. :contentReference[oaicite:1]{index=1}  
- **GitLab**: Uses the GitLab REST API to fetch “pushed” events, aggregates commit counts per day, and returns a sorted history (by date). :contentReference[oaicite:2]{index=2}

---

## How it works

- Providers implement a shared `GitProvider` trait:
  - `get_name() -> String`
  - `get_stats(username, token, url) -> Result<Vec<(String, i64)>, String>` :contentReference[oaicite:3]{index=3}
- The server registers both providers (`GitHub`, `GitLab`) in shared state and injects it into the GraphQL schema. :contentReference[oaicite:4]{index=4}
- Axum serves:
  - `POST /graphql` for GraphQL requests
  - `GET /` for GraphiQL UI :contentReference[oaicite:5]{index=5}

---

## API

### Endpoints

- `GET /` – GraphiQL UI :contentReference[oaicite:6]{index=6}  
- `POST /graphql` – GraphQL endpoint :contentReference[oaicite:7]{index=7}  

Server binds to `0.0.0.0:3000` by default. :contentReference[oaicite:8]{index=8}

### GraphQL schema

The core query is:

- `stats(name: String!, username: String!, token: String!, url: String): Stats`

Where:

- `name`: provider name (e.g. `"GitHub"` or `"GitLab"`) (matched case-insensitively) :contentReference[oaicite:9]{index=9}
- `username`: username used in the path to the current git forge endpoint :contentReference[oaicite:10]{index=10}
- `token`:
  - Private Access Token: **PRIVATE-TOKEN** header :contentReference[oaicite:12]{index=12}
- `url`: required for self-hosted Git instances, ignored for GitHub :contentReference[oaicite:13]{index=13}

The response includes:

- `Stats { history: [DailyContribution!]! }`
- `DailyContribution { date: String!, contributionCount: Int! }` :contentReference[oaicite:14]{index=14}

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

```
::contentReference[oaicite:27]{index=27}
```

[1]: https://raw.githubusercontent.com/Global-Git-Contribution-Graph/global_git_contribution_graph_api/main/Cargo.toml "raw.githubusercontent.com"
[2]: https://raw.githubusercontent.com/Global-Git-Contribution-Graph/global_git_contribution_graph_api/main/src/main.rs "raw.githubusercontent.com"
[3]: https://raw.githubusercontent.com/Global-Git-Contribution-Graph/global_git_contribution_graph_api/main/src/graphql/schema.rs "raw.githubusercontent.com"
[4]: https://raw.githubusercontent.com/Global-Git-Contribution-Graph/global_git_contribution_graph_api/main/.devcontainer/devcontainer.json "raw.githubusercontent.com"
[5]: https://raw.githubusercontent.com/Global-Git-Contribution-Graph/global_git_contribution_graph_api/main/src/providers/github.rs "raw.githubusercontent.com"
[6]: https://raw.githubusercontent.com/Global-Git-Contribution-Graph/global_git_contribution_graph_api/main/src/providers/gitlab.rs "raw.githubusercontent.com"
