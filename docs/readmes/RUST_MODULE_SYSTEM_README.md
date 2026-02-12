# 🦀 Rust Module System: Structural vs. Functional `mod.rs`

This project utilizes two distinct patterns for `mod.rs` files. Understanding the difference is key to mastering **Dioxus Fullstack** and Rust's module system.

---

## 📂 1. The Structural `mod.rs` (Standard Rust)

**Found in:** `src/models/mod.rs`, `src/views/mod.rs`

This file acts as a simple **Gatekeeper**. Its only job is to tell the compiler which files exist in the directory and control their visibility.

```rust
// src/models/mod.rs
pub mod employee; // Points to employee.rs
pub mod auth;     // Points to auth.rs

```

* **Why?** Without this, Rust wouldn't look into the subfiles.
* **Visibility**: Using `pub mod` makes these sub-modules accessible to the rest of the application.

---

## 🌐 2. The Functional `mod.rs` (Dioxus Server Bridge)

**Found in:** `src/server/mod.rs`

This file is a **Bridge**. It doesn't just link files; it defines the boundary between your browser (WASM) and your PostgreSQL database.

### Core Implementation

```rust
use crate::models::employee::Employee;
use dioxus::prelude::*;

// Database connection logic is isolated to the server feature
#[cfg(feature = "server")]
pub mod db;

#[server]
pub async fn get_employees() -> Result<Vec<Employee>, ServerFnError> {
    #[cfg(feature = "server")]
    {
        // 1. Database Connection via the server-only db module
        let pool = crate::server::db::connect_db()
            .await
            .map_err(|e| ServerFnError::new(format!("DB Connection failed: {}", e)))?;

        // 2. Type-safe Query execution using SQLx
        let employees = sqlx::query_as::<_, Employee>(
            "SELECT id, first_name, last_name, email FROM employee ORDER BY id",
        )
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Database query failed: {}", e)))?;

        Ok(employees)
    }

    // Proxy error for client-side compilation
    #[cfg(not(feature = "server"))]
    {
        Err(ServerFnError::new("Server function not available on client"))
    }
}

```

---

## ⚖️ Key Differences

| Feature | Structural `mod.rs` (`models/`) | Functional `mod.rs` (`server/`) |
| --- | --- | --- |
| **Primary Goal** | Code organization. | API definition (RPC). |
| **Macros** | None or standard `pub mod`. | `#[server]` for network bridging. |
| **Conditional Compilation** | Usually unconditional. | Heavy use of `#[cfg(feature = "server")]`. |
| **Security** | Public data shapes. | Encapsulates DB credentials & SQL logic. |

---

## 🛠 Why this matters for Fullstack Rust

1. **Binary Size**: By using `#[cfg(feature = "server")]`, we ensure that heavy database drivers (like `sqlx`) are not compiled into the WebAssembly binary that the user downloads.
2. **Type Safety**: The `#[server]` macro generates a client-side proxy. When you call `get_employees()` in your UI, it looks like a local function but executes as a POST request to your Axum/Tokio backend.
3. **Encapsulation**: Your SQL queries never leave the server. The client only sees the resulting `Vec<Employee>`.
