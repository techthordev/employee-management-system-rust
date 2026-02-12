# Employee Management System (Rust)

A clean and structured **fullstack Employee Management System** built with **Rust**, focused on modern architecture, type-safe database integration, and a unified development workflow.

This project is designed as a portfolio piece demonstrating real-world patterns:
unified fullstack logic, repository patterns, database migrations, and secure authentication.

---

## 🚀 Tech Stack

### Fullstack Framework
- **Rust** with **Dioxus 0.7**
- **Axum** (Server-side integration)
- **Tokio** (Async runtime)

### Database & UI
- **PostgreSQL 18 Alpine**
- **SQLx** (Compile-time verified queries)
- **Tailwind CSS** (Modern responsive UI)

---

## 📁 Project Structure

```txt
employee-management-system-rust/
├── src/                     # Unified Source (Frontend UI + Server Functions)
├── database/                # PostgreSQL init scripts (schema + sample data)
├── docs/                    # Detailed documentation
├── assets/                  # Static assets & Tailwind CSS
├── compose.yml              # Podman Compose configuration
├── Dioxus.toml              # Dioxus CLI settings
├── .env                     # Local environment variables
└── Cargo.toml               # Project dependencies

```

## 📚 Documentation

Detailed documentation is stored in:

📌 `docs/readmes/`

### Available Docs

* [Database Setup](https://www.google.com/search?q=docs/readmes/DATABASE_README.md)
* [SQLx Setup](https://www.google.com/search?q=docs/readmes/SQLX_README.md)
* [GitHub Setup (git + gh)](https://www.google.com/search?q=docs/readmes/GITHUB_README.md)
* [Architecture Decision: Why Dioxus?](https://www.google.com/search?q=docs/readmes/WORKSPACE_README.md)

---

## 🐘 Database Setup (Podman)

The database runs locally via **Podman Compose**.

### Start PostgreSQL

```bash
podman compose up -d

```

### Stop PostgreSQL

```bash
podman compose down

```

---

## 🔐 Environment Variables

Create a `.env` file in the project root:

```env
DATABASE_URL=postgres://rustconnector:rustconnector@localhost:5432/ems_db

```

---

## 🦀 Running the Application

The project uses the **Dioxus CLI** to manage both frontend and backend.

### Start Development Server

```bash
dx serve

```

The app will be available at `http://127.0.0.1:8080` with **Hot Reload** enabled.

---

## 🔄 Reset Database (Hard Reset)

⚠️ This will delete all database data permanently.

```bash
podman compose down
podman volume rm ems_db_data
podman compose up -d

```

---

## 🗺️ Roadmap

### Phase 1 (Core & Foundation) ✅

* [x] PostgreSQL setup (Podman Compose)
* [x] Unified Dioxus Fullstack architecture
* [x] Tailwind CSS integration

### Phase 2 (Backend Logic & Data) 🚧

* [ ] SQLx migrations & compile-time verification
* [ ] Employee CRUD Server Functions
* [ ] Authentication (JWT & RBAC)

### Phase 3 (Frontend & Dashboard) 📅

* [ ] Interactive Dashboard views
* [ ] Role-based UI access control
* [ ] Advanced filtering and reporting

---

## 📌 Notes

By using **Dioxus Fullstack**, we eliminate the need for manual REST boilerplate. We use **Server Functions** to bridge the gap between the browser and PostgreSQL, ensuring 100% type safety across the entire network boundary.

---

## 📜 License

MIT
