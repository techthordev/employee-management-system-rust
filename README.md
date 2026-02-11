# Employee Management System (Rust)

A clean and structured **fullstack Employee Management System** built with **Rust**, focused on modern backend architecture, database design, and scalable project structure.

This project is designed as a learning and portfolio project with real-world patterns:
service layer, repository layer, database migrations, authentication schema, and a future frontend.

---

## 🚀 Tech Stack

### Backend
- **Rust**
- **Axum** (REST API)
- **Tokio** (async runtime)
- **SQLx** (PostgreSQL integration + migrations)

### Database
- **PostgreSQL 18 Alpine**
- Two-schema design (`public` + `auth`)
- Sample data included

### Frontend (planned)
- **Dioxus** (Rust UI framework)

---

## 📁 Project Structure

```txt
employee-management-system-rust/
├── backend/                 # Axum REST API (Rust)
├── frontend/                # Dioxus UI (planned)
├── shared/                  # Shared types (planned)
├── database/                # PostgreSQL init scripts (schema + sample data)
├── compose.yml              # Podman Compose configuration
├── .env                     # local env variables
└── Cargo.toml               # Rust workspace
````

## 📚 Documentation

Detailed documentation is stored in:

📌 `docs/readmes/`

### Available Docs

- [Database Setup](docs/readmes/DATABASE_README.md)
- [GitHub Setup (git + gh)](docs/readmes/GITHUB_README.md)
- [Rust Workspace Setup](docs/readmes/WORKSPACE_README.md)

---

## 🐘 Database Setup (Podman)

The database runs locally via **Podman Compose**.

### Start PostgreSQL

```bash
podman compose up -d
```

### Check status

```bash
podman ps
podman logs -f ems-db-container
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
SERVER_ADDR=127.0.0.1:3000
POSTGRES_PORT=5432
POSTGRES_PASSWORD=postgres
```

---

## 🦀 Running the Backend

```bash
cd backend
cargo run
```

Backend will run on:

```
http://127.0.0.1:3000
```

---

## 🧪 Health Check

```bash
curl http://127.0.0.1:3000/health
```

Expected output:

```
OK
```

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

### Phase 1 (Database + Setup)

* [x] PostgreSQL setup (Podman Compose)
* [x] init scripts (schemas + sample data)

### Phase 2 (Backend API)

* [ ] SQLx migrations
* [ ] Employee CRUD endpoints
* [ ] Department CRUD endpoints
* [ ] Authentication (RBAC)

### Phase 3 (Frontend)

* [ ] Dioxus frontend UI
* [ ] Login + role-based UI access
* [ ] Dashboard views

---

## 📌 Notes

This project is intentionally structured like a real backend system:
controllers/handlers, services, repositories, clean separation of concerns.

The goal is not just CRUD, but maintainable architecture.

---

## 📜 License

MIT (or choose your preferred license)
