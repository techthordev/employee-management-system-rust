# 🦀 Architecture Decision: Dioxus Fullstack

This document outlines the reasoning behind choosing **Dioxus** as the core framework for the Employee Management System.

---

## 📌 Why Dioxus?

Dioxus was selected to leverage the power of **Rust** across the entire stack, providing a seamless bridge between high-performance backend logic and a modern, reactive user interface.

### 1. Single-Language Fullstack Power
Using Rust for both the server and the frontend (via WebAssembly) eliminates the "context switching" between different languages. This ensures:
- **Shared Logic:** Validation rules and data models are defined once and used everywhere.
- **Type Safety:** Changes in the backend API are caught by the compiler in the frontend immediately.

### 2. High Performance (WASM)
By compiling to **WebAssembly**, the frontend delivers near-native performance. This is crucial for data-heavy management systems that require fast sorting, filtering, and rendering of employee records.

### 3. Server Functions (RPC Pattern)
Dioxus allows for an **RPC-like** development experience. Instead of manually defining REST endpoints and fetching them via `JSON` in TypeScript, we use **Server Functions**. This simplifies the architecture while maintaining full control over the communication layer.

---

## 📁 Integrated Project Structure

The project follows a unified structure to maximize efficiency:

- **Frontend:** Reactive UI components built with Rust macros.
- **Backend:** High-performance Axum-based server integration.
- **Styling:** Utility-first CSS using **Tailwind**, integrated into the Rust build pipeline.

---

## ⚙️ Developer Experience (DX)

Choosing Dioxus provides a modern developer workflow on **Fedora**:
- **Hot Reloading:** Instant UI updates without full recompilation.
- **First-Class Tooling:** The Dioxus CLI (`dx`) simplifies building, testing, and deploying the fullstack application.

---

## 📌 Conclusion

The choice of Dioxus aligns with the goal of building a **maintainable, type-safe, and high-performance** system. It represents the "Rust Way" of web development—combining the safety of a compiled language with the agility of modern web frameworks.