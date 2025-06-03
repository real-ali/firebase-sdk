# firebase-core

> A clean, modular, and production-ready Rust package to initialize and manage Firebase apps  
> leveraging a modern Clean Architecture approach for maintainability and scalability.

---

## 🚀 Overview

`firebase-core` provides a robust foundation for integrating Firebase Core functionalities in Rust projects.  
It follows **Clean Architecture** principles to separate domain logic, use cases, and infrastructure, enabling clean code, testability, and flexibility.

This package currently supports:

- Firebase App initialization with custom configurations
- Authentication token management (fetch, refresh)
- HTTP client abstraction for Firebase REST API communication

---

## 📂 Project Structure

firebase-core/
├── src/
│ ├── domain/ # Core entities and domain errors
│ ├── usecases/ # Application business logic (initialization, token management)
│ ├── infrastructure/ # API clients, HTTP adapters, config loaders
│ ├── interface/ # Public-facing API for consumers of the crate
│ ├── errors.rs # Common error definitions
│ ├── lib.rs # Library entry point (re-exports interface)
│ └── main.rs # Optional example / test binary
├── Cargo.toml
└── README.md
