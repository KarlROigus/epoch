# Epoch — Personal Time Tracker

A Rust CLI time tracker for personal productivity.

## Vision

Start with a solid CLI tool with clean business logic, then expand to other interfaces. Includes the full devops side — CI/CD, monitoring, observability.

## Feature Roadmap

### Phase 1 — Core Tracking (MVP)
- [ ] Start/stop timer (`epoch start`, `epoch stop`)
- [ ] Name/describe what you're working on (`epoch start "writing docs"`)
- [ ] Show current running timer (`epoch status`)
- [ ] Manual time entry (`epoch add`)
- [ ] Edit/delete entries
- [ ] List today's entries (`epoch log`)

### Phase 2 — Organization
- [ ] Projects — group entries under projects
- [ ] Tags — flexible labeling (`epoch start "code review" --tag work --tag rust`)
- [ ] Categories

### Phase 3 — Reporting
- [ ] Daily/weekly/monthly summaries
- [ ] Per-project breakdowns
- [ ] Export to CSV/JSON

### Phase 4 — UX Niceties
- [ ] Idle detection
- [ ] Reminders ("you've been working for 2h, take a break")
- [ ] Pomodoro mode
- [ ] Shell completions (zsh/bash/fish)

### Phase 5 — Integrations
- [ ] Calendar sync
- [ ] GitHub commit correlation
- [ ] Slack status updates

### Phase 6 — DevOps & Observability
- [ ] CI/CD pipeline
- [ ] Metrics / monitoring
- [ ] Structured logging
- [ ] Release automation

### Phase 7 — Beyond CLI
- [ ] REST API server
- [ ] Web frontend
- [ ] TUI (terminal UI with ratatui or similar)
- [ ] Sync across devices

## Non-Goals
- Billing / invoicing
- Team management
