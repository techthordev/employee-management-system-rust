# 🦀 SQLx Integration: Type-Safe Database Access

This document details how **SQLx** is integrated into the Employee Management System to provide async, type-safe, and compile-time verified database operations.

---

## 🛠 Why SQLx?

Unlike traditional ORMs (like Diesel or Hibernate), **SQLx** allows us to write raw SQL while still maintaining 100% type safety.

* **Async by Default:** Built to work seamlessly with the `tokio` runtime used by Dioxus.
* **Compile-Time Verification:** SQLx connects to your live database during compilation to verify that your queries are syntactically correct and match your schema.
* **No DSL:** You write standard PostgreSQL, making it easy to optimize and debug.

---

## 📦 Configuration & Dependencies

The following dependencies are configured in `Cargo.toml`:

```toml
[dependencies]
sqlx = { version = "0.8", features = ["runtime-tokio", "tls-rustls", "postgres", "macros", "chrono"] }
dotenvy = "0.15" # To load DATABASE_URL from .env

```

### Environment Variables

SQLx requires a `DATABASE_URL` to verify queries. This is stored in your `.env` file:

```env
DATABASE_URL=postgres://rustconnector:rustconnector@localhost:5432/ems_db

```

---

## 🏗 Implementation Details

### 1. The Database Connection Pool (`src/server/db.rs`)

To avoid opening a new connection for every request, we use a `PgPool`. This pool is initialized once and shared across server functions.

```rust
pub async fn connect_db() -> Result<sqlx::PgPool, anyhow::Error> {
    dotenvy::dotenv().ok(); // Load .env file
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    let pool = sqlx::PgPool::connect(&database_url).await?;
    Ok(pool)
}

```

### 2. Type Mapping with `FromRow` (`src/models/employee.rs`)

We use the `FromRow` derive macro to automatically map database columns to Rust struct fields.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))] // Only derived on server-side
pub struct Employee {
    pub id: i64,          // Matches BIGINT or SERIAL
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

```

---

## 🔍 Database Operations (Server Functions)

In `src/server/mod.rs`, we bridge the gap between Dioxus and SQLx.

### Fetching Data (`SELECT`)

We use `query_as` to map the result directly into our `Employee` struct.

```rust
let employees = sqlx::query_as::<_, Employee>(
    "SELECT id, first_name, last_name, email FROM employee ORDER BY id"
)
.fetch_all(&pool)
.await
.map_err(|e| ServerFnError::new(format!("DB Error: {}", e)))?;

```

---

## 🛡 Performance & Safety Features

### 1. Shared State (Coming Soon)

To optimize performance, we will move the `PgPool` into Dioxus **Global State** or **Context**, ensuring the server function doesn't re-connect to the DB on every single call.

### 2. Compile-Time Checked Queries (Recommended)

By using the `sqlx::query!` macro instead of `query_as`, the compiler will check your SQL against the real DB schema during `cargo build`.

* If you rename a column in Postgres but forget to change the Rust code, **the build will fail**.
* This eliminates runtime "Column Not Found" errors entirely.

---

## 🚦 Common Workflow: Adding a Migration

Since we use **Podman** for the database, the workflow for schema changes is:

1. Add a new `.sql` file to `database/schema/`.
2. Update your Rust `models/`.
3. Perform a **Hard Reset** of the container to apply changes:
```bash
podman compose down
podman volume rm ems_db_data
podman compose up -d

```



---

## 📌 Summary of our SQLx Setup

| Feature | Implementation |
| --- | --- |
| **Driver** | PostgreSQL (Pure Rust `rustls`) |
| **Mapping** | `sqlx::FromRow` for automatic struct binding |
| **Runtime** | `tokio` (Async) |
| **Verification** | Offline/Online check via `DATABASE_URL` |
| **Serialization** | Integrated with `serde` for Dioxus Frontend |

---

**Last Updated:** 2026-02-12
**SQLx Version:** 0.8.x
