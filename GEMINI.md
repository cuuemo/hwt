# GEMINI.md - AT Network Maintenance System

This document provides an overview and development guide for the AT (网维系统) project, serving as the foundational context for Gemini CLI interactions.

## Project Overview

AT is a specialized network maintenance system designed for internet cafes and computer rooms. Its primary function is to clean monitor registry entries and randomize machine identifiers to ensure system stability and performance. The system features a three-tier architecture with cloud-based authorization, a local maintenance server, and workstation clients.

### Architecture
- **Cloud Server (Internet):** A Python FastAPI backend with a Vue 3 frontend for user registration, license management (monthly, yearly, permanent), and machine binding/unbinding.
- **Maintenance Server (Local LAN):** A Rust application (`at-server`) that authenticates with the cloud, listens for workstation connections on TCP 19800, and provides a local Web UI (Axum) for monitoring and management.
- **Workstation Client (Local LAN):** A Rust-based Windows Service (`at-client`) that runs under SYSTEM privileges. It auto-discovers the maintenance server via LAN scanning, authenticates via encrypted channels, and performs device cleanup using the Win32 Setup API and registry modifications.

## Key Technologies
- **Rust:** Used for performance-critical and system-level components (Client, Server, Protocol).
- **Python (FastAPI):** Powers the cloud backend API.
- **Vue 3 + TypeScript:** Used for the administrative dashboard.
- **Cryptography:** RSA OAEP-SHA256 for handshakes and AES-256-GCM for secure session communication.
- **Database:** SQLite (default) via SQLAlchemy and Alembic for the cloud backend.
- **Internationalization (i18n):** Integrated `vue-i18n`. Element Plus components use `<el-config-provider>` for dynamic locale switching.
- **Styling:** Modern dark cyber theme using CSS variables (`theme.css`). Authentication pages share `auth.css`.
- **Utilities:** Shared formatting logic in `src/utils/`.

## Project Structure

- `client/`: Rust source for the Windows Service client.
- `server/`: Rust source for the maintenance server application.
- `protocol/`: Shared Rust library for communication protocols and cryptography.
- `cloud/backend/`: Python FastAPI service.
- `cloud/frontend/`: Vue 3 administrative dashboard.
- `demo/`: A simplified demonstration version of the system.
- `DeviceCleanup_extracted/`: Research data and decompiled code from the `DeviceCleanup` tool.
- `build.sh`: Main script for cross-compiling the Rust components for Windows.

## Building and Running

### Rust Workspace (Client & Server)
- **Build All:** `cargo build --workspace`
- **Cross-compile for Windows:** `cargo build --release --target x86_64-pc-windows-gnu -p at-server`
- **Release Build Script:** `bash build.sh` (Produces IP and domain-based variants in `dist/`).
- **Client Service Management:**
  - Install: `at-client.exe install`
  - Uninstall: `at-client.exe uninstall`
  - Status: `at-client.exe status`

### Cloud Backend
- **Development:** `cd cloud/backend && bash start.sh --reload` (Runs on port 10000).
- **Docker:** `cd cloud/backend && docker compose up -d`
- **Testing:** `cd cloud/backend && pytest`

### Cloud Frontend
- **Install Dependencies:** `cd cloud/frontend && npm install`
- **Development:** `npm run dev`
- **Production Build:** `npm run build`

## Development Conventions

- **Indentation:** 4-space indentation for Rust, Python, and TypeScript.
- **Commits:** Use Conventional Commits (`feat:`, `fix:`, `docs:`, `chore:`).
- **Naming Conventions:**
  - **Rust:** `snake_case` for functions/variables, `CamelCase` for types.
  - **Python:** Lowercase module names.
  - **Vue:** `PascalCase.vue` for views and layouts.
- **Web UI:** Both server and client binaries embed their respective Web UIs using `include_str!()`. Axum is used to serve these interfaces on ports 19880 (Server) and 19881 (Client).
- **Security:** Never commit RSA keys (`cloud/backend/keys/`), database files, or sensitive credentials. The `JWT_SECRET` and admin passwords must be changed before deployment.

## Common Tasks
- **Updating the Protocol:** Changes to `protocol/src/lib.rs` must be reflected in both the `client` and `server` crates.
- **Adding Cleanup Logic:** New registry keys or device classes should be added to `client/src/registry.rs` or `client/src/cleanup.rs`.
- **Modifying the UI:** Frontend changes are made in `cloud/frontend/src/` or the embedded HTML/CSS in the Rust crates' `assets/` directories.
