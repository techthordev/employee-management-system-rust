# Employee Management System (Rust)

PostgreSQL database configuration for the Employee Management System (Rust version).

This project uses **Podman Compose** to run PostgreSQL locally.

---

## 📁 Structure

```bash
database/
├── init/
│   └── 01-init.sql                    # User & database privileges
├── schema/
│   ├── 02-default-schema.sql          # Business tables (departments, employee, etc.)
│   └── 03-auth-schema.sql             # Authentication schema (auth.users, auth.authorities)
├── data/
│   ├── 04-employee-data.sql           # Sample employee records
│   └── 05-auth-data.sql               # Sample auth users + roles
└── reset/
    └── reset.sql                      # Reset script for development
````

---

## 🐘 Database Info

| Setting        | Value                             |
| -------------- | --------------------------------- |
| **Database**   | `ems_db`                          |
| **Admin User** | `postgres` / `postgres`           |
| **App User**   | `rustconnector` / `rustconnector` |
| **Port**       | `5432` (configurable via `.env`)  |
| **Version**    | PostgreSQL 18 Alpine              |

---

## 🏗️ Schema Architecture

### **Two-Schema Design**

### `auth` Schema (Authentication & Authorization)

* `auth.users` - User accounts for login
* `auth.authorities` - Role assignments (RBAC)
* Owned by: `postgres`
* Access: `rustconnector` has limited access (depending on privileges)

### `public` Schema (Business Data)

* `departments` - Company departments
* `employee` - Employee records with department assignment
* Owned by: `rustconnector`
* Access: Full CRUD for `rustconnector`

---

## 👥 Default Users

| Username      | Password   | Role               | Description                     |
| ------------- | ---------- | ------------------ | ------------------------------- |
| `admin`       | `password` | `ROLE_ADMIN`       | System Administrator            |
| `hr.admin`    | `password` | `ROLE_HR_ADMIN`    | HR Department Chief (Full CRUD) |
| `hr.manager`  | `password` | `ROLE_HR_MANAGER`  | Senior HR (CRUD except Delete)  |
| `hr.employee` | `password` | `ROLE_HR_EMPLOYEE` | Junior HR (Read-Only)           |

**Note:** Password is a BCrypt hash of `"password"` - **change in production!**

---

## 📊 Sample Data

### Departments

* Human Resources (HR)
* Information Technology (IT)
* Finance (FIN)
* Sales (SAL)
* Marketing (MKT)
* Operations (OPS)
* Customer Support (SUP)
* Research & Development (RND)

### Employees

Sample employees distributed across all departments.

---

## 🚀 Quick Start

### 1. Start Database

```bash
podman compose up -d
```

### 2. Check Status

```bash
podman ps
podman compose ps
```

### 3. View Logs

```bash
podman logs -f ems-db-container
```

---

## 🔍 Verify Installation

### Check schemas

```bash
podman exec -it ems-db-container psql -U postgres -d ems_db -c "\dn"
```

### Check departments

```bash
podman exec -it ems-db-container psql -U postgres -d ems_db \
  -c "SELECT * FROM departments;"
```

### Check auth users and roles

```bash
podman exec -it ems-db-container psql -U postgres -d ems_db -c "
SELECT u.username, STRING_AGG(a.authority, ', ') AS roles
FROM auth.users u
JOIN auth.authorities a ON u.username = a.username
GROUP BY u.username
ORDER BY u.username;"
```

### Check employee count

```bash
podman exec -it ems-db-container psql -U postgres -d ems_db \
  -c "SELECT COUNT(*) FROM employee;"
```

---

## 🔌 Connect to Database

### As postgres admin

```bash
podman exec -it ems-db-container psql -U postgres -d ems_db
```

### As application user

```bash
podman exec -it ems-db-container psql -U rustconnector -d ems_db
```

---

## 🔧 Common Podman Commands

### Container Management

```bash
# Start
podman compose up -d

# Stop
podman compose down

# Restart
podman compose restart

# Check status
podman compose ps

# View logs
podman logs -f ems-db-container

# List containers
podman ps

# List all containers (including stopped)
podman ps -a
```

---

## 💾 Volume Information

The database uses a persistent Podman volume:

* `ems_db_data`

This volume stores all PostgreSQL data.

---

## 🔄 Reset Database

### Soft Reset (keep structure, reload data)

```bash
podman exec -i ems-db-container psql -U postgres -d ems_db \
  < database/reset/reset.sql
```

### Hard Reset (destroy everything)

⚠️ WARNING: This will permanently delete all data.

```bash
podman compose down
podman volume rm ems_db_data
podman compose up -d
```

**Note:** Hard reset will re-run all initialization scripts.

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

## 🐛 Troubleshooting

### Port already in use

```bash
sudo lsof -i :5432
```

Or change port:

```env
POSTGRES_PORT=5433
```

Then restart:

```bash
podman compose down
podman compose up -d
```

---

### Container won't start

```bash
podman logs ems-db-container
```

If needed, reset volume:

```bash
podman compose down
podman volume rm ems_db_data
podman compose up -d
```

---

### Permission denied (SELinux - Fedora/RHEL)

If SELinux blocks mounted SQL scripts:

```bash
chcon -R -t container_file_t database/
```

The compose file uses `:Z` already, which usually solves this.

---

### Scripts not executing

Init scripts only run on first startup (empty volume).

To force re-run:

```bash
podman compose down
podman volume rm ems_db_data
podman compose up -d
```

---

## 📝 Notes

* Initialization scripts run **only on first start** (when the volume is empty)
* Scripts execute in **alphabetical order** (`01`, `02`, `03`, ...)
* Healthcheck ensures PostgreSQL is ready before the backend connects
* All timestamps should be treated as UTC
* This database setup is independent from the Rust backend implementation

---

## 🔗 Related Documentation

* [PostgreSQL 18 Documentation](https://www.postgresql.org/docs/18/)
* [Podman Compose](https://github.com/containers/podman-compose)

---

**Last Updated:** 2026-02-11
**PostgreSQL Version:** 18 Alpine
**Schema Version:** 1.0

```

