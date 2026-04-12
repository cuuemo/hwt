# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

AT (网维系统) is an internet cafe management system with three components:
- **Rust workspace** (`protocol/`, `client/`, `server/`): Windows executables for LAN device management
- **Cloud backend** (`cloud/backend/`): Python FastAPI service for auth, licensing, and machine binding
- **Admin frontend** (`cloud/frontend/`): Vue 3 + TypeScript + Element Plus dashboard

Architecture: Client (Windows Service) → Server (silent background + Web UI) → Cloud (HTTPS API). Both server and client run silently and are controlled via embedded web UIs (axum + WebSocket). Clients auto-discover the server via LAN scan on TCP 19800, authenticate through RSA+AES encrypted channels, and perform device cleanup. The server authenticates with the cloud using RSA handshake + AES session keys.

## Build & Development Commands

```bash
# Rust workspace
cargo build --workspace                    # build all crates
cargo test --workspace                     # run all Rust tests
cargo build --release --target x86_64-pc-windows-gnu -p at-server  # cross-compile for Windows
bash build.sh                              # build Windows release binaries (IP + domain variants) into dist/

# Cloud backend
cd cloud/backend && bash start.sh --reload # run FastAPI dev server on port 10000
cd cloud/backend && pytest                 # run backend tests
cd cloud/backend && pytest tests/test_auth.py -k test_name  # run single test

# Admin frontend
cd cloud/frontend && npm run dev           # Vite dev server
cd cloud/frontend && npm run build         # type-check + production build
```

The `CLOUD_BASE_URL` env var controls which cloud endpoint the server binary connects to (set at compile time via `build.sh`).

## Code Style

- 4-space indentation across Rust, Python, and TypeScript
- Conventional Commits: `feat:`, `fix:`, `docs:`, `chore:`
- Rust: `snake_case` functions, `CamelCase` types, crate names follow `at-*` pattern
- Python: lowercase modules, test helpers in `conftest.py`
- Vue: `PascalCase.vue` for views/layouts, API helpers under `src/api/`

## Architecture Details

### Web UI (axum)
Both server and client embed an axum web server with WebSocket support. HTML/CSS/JS assets are compiled into the binary via `include_str!()`. The server web UI on port 19880 has cookie-based session auth (`at_session`, HttpOnly). The client web UI on port 19881 is unauthenticated (status-only, no sensitive data). Both support EN/CN language toggle (persisted in localStorage). Rust `log` output is bridged to the broadcast channel and appears in the web UI log area.

### Crypto Protocol
`protocol/` crate provides shared crypto (RSA OAEP-SHA256 + AES-256-GCM) and TCP frame encoding used by both client and server. The cloud backend implements the same protocol in Python (`cloud/backend/app/crypto.py`).

### Server builds two variants
`build.sh` produces IP-based and domain-based builds by setting `CLOUD_BASE_URL` env var before each compilation. Both share the same client binary.

### Client runs as Windows Service
`client/` installs as a Windows Service running under SYSTEM. It scans LAN port 19800 for the server, performs RSA handshake, then executes display registry cleanup and ghost device removal via Win32 Setup API.

### Backend database
SQLite via SQLAlchemy with Alembic migrations. Models: User, Binding, Log. JWT-based auth with admin role for management operations.

## Security

Do not commit RSA keys (`cloud/backend/keys/`), database files (`cloud/backend/data/`), or credentials. Change `JWT_SECRET` and admin password in `docker-compose.yml` before deployment.
