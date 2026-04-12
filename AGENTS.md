# Repository Guidelines

## Project Structure & Module Organization
This repository combines a Rust workspace, a Python cloud backend, and a Vue admin frontend.

- `client/`: Windows service client (`at-client`) that scans the LAN, authenticates, and performs cleanup. Embeds a web UI on port 19881 (axum + WebSocket).
- `server/`: Windows server (`at-server`) running silently with embedded web UI on port 19880 (axum + WebSocket, cookie-based session auth).
- `protocol/`: shared Rust protocol and crypto primitives used by client and server.
- `cloud/backend/`: FastAPI service, static admin assets, Docker files, and pytest suite in `cloud/backend/tests/`.
- `cloud/frontend/`: Vite + Vue 3 + TypeScript admin UI.
- `dist/`, `target/`, `cloud/frontend/dist/`, and `cloud/frontend/node_modules/` are build artifacts and should not be edited directly.

## Build, Test, and Development Commands
- `cargo build --workspace`: build the Rust workspace for local development.
- `cargo test --workspace`: run Rust tests across `client`, `server`, and `protocol`.
- `bash build.sh`: produce Windows release binaries in `dist/ip/` and `dist/domain/`. This script temporarily rewrites `server/src/auth.rs`; review that file before committing.
- `cd cloud/backend && bash start.sh --reload`: run the FastAPI backend locally on port `10000`.
- `cd cloud/backend && pytest`: run backend tests.
- `cd cloud/frontend && npm run dev`: start the Vite dev server.
- `cd cloud/frontend && npm run build`: type-check and build the frontend bundle.

## Coding Style & Naming Conventions
Use 4-space indentation across Rust, Python, and TypeScript/Vue files. Follow Rust 2021 defaults: `snake_case` for functions/modules, `CamelCase` for types, and keep crate names aligned with existing `at-*` patterns. In Python, keep modules lowercase and test helpers in `conftest.py`. In Vue, use `PascalCase.vue` for views and layouts, and keep API helpers under `src/api/`.

## Testing Guidelines
Add Rust unit tests close to the code they verify, especially in `protocol/` for shared logic. Backend tests belong in `cloud/backend/tests/` and should follow the existing `test_*.py` naming pattern. Run `cargo test --workspace` and `pytest` before opening a PR; if frontend behavior changes, also run `npm run build`.

## Commit & Pull Request Guidelines
Recent history uses Conventional Commit prefixes such as `feat:`, `fix:`, and `docs:`. Keep subjects imperative and scoped to one change. PRs should describe the affected area, list verification commands, link related issues, and include screenshots for GUI or admin UI changes.

## Security & Configuration Tips
Do not commit real credentials, generated keys, or populated data directories. For cloud deployment, change `JWT_SECRET` and admin credentials in `cloud/backend/docker-compose.yml` before release.
