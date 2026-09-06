# Epoch — Phase 1 MVP Plan

## Context

Epoch is a personal time tracker built in Rust. The goal is a working CLI → server → Postgres pipeline for core time tracking. Phase 1 gets the vertical slice working end-to-end: you can start a timer, stop it, see what's running, and view your entries. Auth is simple for now (static API key) — the separate auth service with JWT/expiry comes in Phase 2.

## Repo Structure

```
epoch/
├── Cargo.toml              ← workspace root
├── cli/                    ← epoch-cli crate (binary)
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── server/                 ← epoch-server crate (binary)
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── routes/         ← axum route handlers
│       ├── db/             ← postgres queries
│       └── models/         ← shared types
├── common/                 ← epoch-common crate (library)
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs          ← request/response types shared between cli and server
├── migrations/             ← SQL migration files
├── containers/             ← k8s manifests (Phase 2+)
├── .github/                ← CI workflows (Phase 2+)
├── project.md
└── README.md
```

## Database Schema

Single table to start:

```sql
CREATE TABLE entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    description TEXT NOT NULL,
    started_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    stopped_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

A running timer = a row where `stopped_at IS NULL`. Only one can exist at a time (enforced in application logic).

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| POST | `/api/timer/start` | Start a new timer `{ "description": "..." }` |
| POST | `/api/timer/stop` | Stop the running timer |
| GET | `/api/timer/status` | Get the currently running timer (if any) |
| GET | `/api/entries` | List entries (default: today) |
| DELETE | `/api/entries/:id` | Delete an entry |
| PUT | `/api/entries/:id` | Edit an entry |

Auth: static API key passed as `Authorization: Bearer <key>` header. Key configured via env var on the server, stored in `~/.epoch/config.toml` on the client.

## CLI Commands

```
epoch start "writing docs"    → POST /api/timer/start
epoch stop                    → POST /api/timer/stop
epoch status                  → GET  /api/timer/status
epoch log                     → GET  /api/entries (today)
epoch delete <id>             → DELETE /api/entries/:id
epoch edit <id> --desc "..."  → PUT /api/entries/:id
```

No CLI framework (no clap) — manual arg parsing from `std::env::args()`.

## Client Config

`~/.epoch/config.toml`:
```toml
server_url = "http://localhost:3000"
api_key = "your-secret-key"
```

## Dependencies

**server:**
- `axum` — HTTP routing
- `tokio` — async runtime
- `sqlx` — async Postgres driver (compile-time checked queries)
- `serde` / `serde_json` — serialization
- `uuid` — entry IDs
- `chrono` — timestamps

**cli:**
- `reqwest` (blocking) — HTTP client
- `serde` / `serde_json` — serialization
- `toml` — config parsing

**common:**
- `serde` — shared request/response types
- `uuid`, `chrono` — shared types

## Implementation Steps

### Step 1 — Scaffold the workspace
- `Cargo.toml` workspace with members: `cli`, `server`, `common`
- Each crate with minimal `Cargo.toml` and placeholder `main.rs` / `lib.rs`
- `.gitignore` for Rust

### Step 2 — Common types
- Define shared request/response structs in `common/`
- `StartRequest`, `EntryResponse`, `StatusResponse`, etc.

### Step 3 — Database layer
- Write the SQL migration in `migrations/`
- Implement db module in server: `create_entry`, `stop_entry`, `get_running`, `list_entries`, `delete_entry`, `update_entry`

### Step 4 — Server routes
- Axum router with all 6 endpoints
- API key middleware
- Server reads config from env vars: `DATABASE_URL`, `API_KEY`, `PORT`

### Step 5 — CLI client
- Config file parsing (`~/.epoch/config.toml`)
- Arg parsing and dispatch to HTTP calls
- Human-readable output formatting (durations, timestamps)

### Step 6 — Local dev setup
- Document how to run: `docker run postgres` + `cargo run -p epoch-server` + `cargo run -p epoch-cli`

## Verification

1. Start local Postgres: `docker run -e POSTGRES_PASSWORD=epoch -p 5432:5432 postgres`
2. Run migrations
3. Start server: `cargo run -p epoch-server`
4. Test the flow:
   - `epoch start "testing the MVP"` → returns entry with running status
   - `epoch status` → shows running timer with elapsed time
   - `epoch stop` → stops timer, shows duration
   - `epoch log` → shows today's entries
   - `epoch start "another task"` while one is running → error
   - `epoch delete <id>` → removes entry
