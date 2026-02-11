# Rust Workspace Setup

This project is organized as a **Rust Cargo Workspace**.

A workspace is used to manage multiple crates (backend, frontend, shared) inside one repository
while keeping dependencies and build commands consistent.

---

## 📌 Why a Workspace?

Using a workspace provides:

- one central `Cargo.lock`
- shared dependency management
- clean separation between backend / frontend / shared code
- scalable project structure for fullstack development
- easier CI/CD setup later

This is the standard structure for professional Rust monorepos.

---

## 📁 Workspace Structure

```txt
employee-management-system-rust/
├── Cargo.toml               # Workspace root
├── Cargo.lock               # Shared dependency lockfile
├── backend/                 # Backend REST API crate (Axum)
├── frontend/                # Frontend crate (Dioxus, planned)
├── shared/                  # Shared library crate (planned)
└── target/                  # Build artifacts (ignored in git)
````

---

## ⚙️ Workspace Root Cargo.toml

The root `Cargo.toml` contains the workspace definition:

```toml
[workspace]
members = [
    "backend",
    "frontend",
    "shared"
]
```

---

## 🦀 Building the Workspace

Build all crates:

```bash
cargo build
```

Run all tests:

```bash
cargo test
```

---

## ▶️ Running Individual Crates

### Run backend

```bash
cargo run -p backend
```

or:

```bash
cd backend
cargo run
```

---

## 📦 Adding Dependencies

### Add dependency to backend crate

```bash
cd backend
cargo add axum
```

---

## 🔁 Common Commands

### Check formatting

```bash
cargo fmt
```

### Run linter (Clippy)

```bash
cargo clippy
```

### Build release version

```bash
cargo build --release
```

---

## ⚠️ Common Workspace Issue

### Error: "failed to load manifest for workspace member"

Example:

```
failed to load manifest for workspace member frontend
Caused by: No such file or directory
```

This happens if the workspace root references a crate folder that does not exist yet.

Fix:

* Create the missing crate folder:

```bash
cargo new frontend
cargo new shared --lib
```

or

* Remove the crate from the `members` list in `Cargo.toml`.

---

## 📌 Notes

* The workspace is designed for a long-term scalable fullstack project.
* `backend` will contain the REST API and business logic.
* `frontend` will later contain the Dioxus UI.
* `shared` will contain shared DTOs/types used by both backend and frontend.
