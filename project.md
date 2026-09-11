# Epoch — Personal Time Tracker

A Rust CLI time tracker for personal productivity and a vehicle for learning production systems engineering.

## Vision

Start with a solid CLI tool with clean business logic, then expand to other interfaces. Build the full production stack — CI/CD, monitoring, observability, auth — implementing as much as possible from scratch to deeply understand each layer.

## Architecture

```
Mac (any computer)                    VPS (Hetzner, microk8s)
──────────────────                    ──────────────────────
epoch CLI                             traefik (ingress) + TLS
  └─► HTTPS ────────────────────►     epoch-server (axum)
                                        └─► Postgres
```

- **CLI** — thin Rust client, sends HTTP requests
- **Server** — Rust/axum REST API, runs in Kubernetes
- **Postgres** — data store, only reachable inside the cluster
- **Traefik** — reverse proxy / ingress (external dependency)
- **cert-manager** — automatic TLS certificates (external dependency)
- **ghcr.io** — Docker image registry

## Current State

### Done
- [x] Core tracking: `epoch start`, `epoch stop "desc"`, `epoch status`, `epoch log`
- [x] Edit/delete entries
- [x] Server deployed on VPS with microk8s
- [x] Postgres running in Kubernetes
- [x] HTTPS via Let's Encrypt + cert-manager
- [x] Static API key auth
- [x] Docker image on ghcr.io
- [x] VPS hardened (SSH keys only, firewall, auto-updates)

### Not yet done
- [ ] Rename binary from `epoch-cli` to `epoch`
- [ ] CI/CD pipeline (GitHub Actions)
- [ ] Manual time entry (`epoch add`)

## Feature Roadmap

### Phase 2 — Auth Service (build from scratch)
- [ ] Separate auth microservice in Rust
- [ ] User registration and login
- [ ] JWT tokens with expiry
- [ ] Refresh token flow
- [ ] Server validates tokens against auth service

### Phase 3 — Organization
- [ ] Projects — group entries under projects
- [ ] Tags — flexible labeling (`epoch stop "code review" --tag work --tag rust`)
- [ ] Categories

### Phase 4 — Reporting
- [ ] Daily/weekly/monthly summaries
- [ ] Per-project breakdowns
- [ ] Export to CSV/JSON

### Phase 5 — UX Niceties
- [ ] Idle detection
- [ ] Reminders ("you've been working for 2h, take a break")
- [ ] Pomodoro mode
- [ ] Shell completions (zsh/bash/fish)

### Phase 6 — Integrations
- [ ] Calendar sync
- [ ] GitHub commit correlation
- [ ] Slack status updates

### Phase 7 — Beyond CLI
- [ ] Web frontend
- [ ] TUI (terminal UI with ratatui or similar)

## DevOps & Infrastructure Roadmap

### Done
- [x] Docker image build and push to ghcr.io
- [x] Kubernetes manifests in containers/
- [x] VPS security hardening

### Next
- [ ] CI/CD pipeline — GitHub Actions: test → build → push → deploy
- [ ] Monitoring — Prometheus + Grafana (learn the stack, then consider building custom metrics collector)
- [ ] Structured logging — centralized log aggregation
- [ ] Release automation — versioned tags, changelog

## Build-It-Yourself Challenges

Things currently handled by external dependencies that would be valuable to implement from scratch as learning exercises:

| What | Currently | Build yourself |
|------|-----------|----------------|
| **Reverse proxy** | Traefik (ingress) | Write a Rust TCP proxy that terminates TLS, reads Host header, forwards to backend |
| **Certificate management** | cert-manager + Let's Encrypt | Implement the ACME protocol client — HTTP challenges, certificate renewal |
| **DNS resolution (in-cluster)** | CoreDNS | Write a simple DNS server that resolves service names to pod IPs |
| **Storage provisioner** | hostpath-provisioner | Create directories and bind-mount them into containers on PVC requests |
| **Auth service** | Static API key | JWT issuing/validation, refresh tokens, password hashing (planned for Phase 2) |
| **Metrics collection** | Prometheus (not yet installed) | Build a custom metrics collector that scrapes /metrics endpoints |
| **Log aggregation** | Not yet set up | Build a log shipper that tails container logs and stores them centrally |
| **CI/CD runner** | GitHub Actions (not yet set up) | Build a webhook listener that runs tests and deploys on git push |
| **Database backup service** | Manual pg_dump / cron | Build a Rust service that runs scheduled pg_dumps, compresses, uploads to S3, and enforces retention policy |

## Non-Goals
- Billing / invoicing
- Team management
