# GitHub Setup (Git + GitHub CLI)

This document describes how to initialize a Rust project as a Git repository,
create a GitHub repository using the **GitHub CLI (`gh`)**, and push everything cleanly.

---

## ✅ Requirements

Make sure you have installed:

- `git`
- `gh` (GitHub CLI)

Check versions:

```bash
git --version
gh --version
````

---

## 🔐 GitHub Login (gh)

Before creating repositories, login to GitHub:

```bash
gh auth login
```

Verify login:

```bash
gh auth status
```

---

## 📦 Initialize Git Repository

Inside your project root:

```bash
git init
```

Check status:

```bash
git status
```

---

## 🚫 Add .gitignore

Create a `.gitignore` file:

```gitignore
# Rust
/target
**/target

# Env
.env

# OS / editor
.DS_Store
.idea/
.vscode/

# Logs
*.log
```

---

## ⚠️ Fix: "does not have a commit checked out"

If you see an error like:

```
error: 'backend/' does not have a commit checked out
fatal: adding files failed
```

Then one of your subfolders (like `backend/`, `frontend/`, `shared/`) contains its own `.git` folder.

Check:

```bash
ls -la backend
```

If you see `.git`, remove it:

```bash
rm -rf backend/.git
rm -rf frontend/.git
rm -rf shared/.git
```

This ensures your project is managed as a single workspace repository.

---

## ➕ Add Files and Commit

Stage everything:

```bash
git add .
```

Commit:

```bash
git commit -m "Initial project setup"
```

---

## 🌍 Create GitHub Repository with gh

Create a public repository:

```bash
gh repo create employee-management-system-rust --public --source=. --remote=origin --push
```

Create a private repository:

```bash
gh repo create employee-management-system-rust --private --source=. --remote=origin --push
```

---

## 🔍 Verify Remote

```bash
git remote -v
```

---

## 🏷️ Add GitHub Topics (Tags)

Topics improve repository visibility and organization.

Example:

```bash
gh repo edit --add-topic rust \
             --add-topic axum \
             --add-topic sqlx \
             --add-topic postgresql \
             --add-topic podman \
             --add-topic docker-compose \
             --add-topic backend \
             --add-topic rest-api \
             --add-topic dioxus \
             --add-topic fullstack \
             --add-topic employee-management
```

---

## 📝 Set Repository Description

```bash
gh repo edit --description "Employee Management System built with Rust, Axum, SQLx and PostgreSQL (Podman Compose)."
```

---

## 🏷️ Create Git Tag (Versioning)

Create an annotated tag:

```bash
git tag -a v0.1.0 -m "Phase 1: Database setup + workspace structure"
```

Push the tag:

```bash
git push origin v0.1.0
```

---

## 🚀 Create GitHub Release (Optional)

Create a release from the tag:

```bash
gh release create v0.1.0 \
  --title "v0.1.0 - Database Setup" \
  --notes "Initial release: workspace structure, Podman PostgreSQL setup, init scripts."
```

---

## 📌 Recommended Workflow

After this setup, your workflow should look like:

```bash
git add .
git commit -m "Describe your changes"
git push
```

For new versions:

```bash
git tag -a v0.2.0 -m "New features"
git push origin v0.2.0
gh release create v0.2.0 --title "v0.2.0" --notes "..."
```

---

## ✅ Result

After completing these steps you will have:

* a clean Git repository
* a connected GitHub repository
* proper GitHub topics (tags)
* versioning via Git tags
* optional GitHub releases
