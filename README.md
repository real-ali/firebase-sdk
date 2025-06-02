# 🔥 Firebase SDK for Rust

A modular, complete **Firebase SDK written in Rust** — designed for performance, safety, and developer ergonomics.  
Each Firebase service is available as a standalone crate and can be used independently or as part of the unified `firebase-sdk`.

---

## 🧱 Workspace Structure

This is a Cargo [workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) containing all Firebase modules:

```
firebase-sdk/
├── firebase-sdk/               # Unified SDK re-exporting all core crates
└── crates/
    ├── firebase-core/          # Shared types, configs, and auth base
    ├── firebase-authentication/ # Firebase Auth API
    ├── firebase-firestore/     # Firestore (NoSQL DB)
    ├── firebase-realtime-db/   # Realtime Database
    ├── firebase-messaging/     # FCM (Firebase Messaging)
    └── firebase-store/         # Firebase Storage (file uploads)
```

---

## 📦 Crates

| Crate                       | Description                                  | Version |
|----------------------------|----------------------------------------------|---------|
| [`firebase-core`](https://crates.io/crates/firebase-core) | Shared internal logic & traits             | `0.1.x` |
| [`firebase-authentication`](https://crates.io/crates/firebase-authentication) | Firebase Authentication API                | `0.1.x` |
| [`firebase-firestore`](https://crates.io/crates/firebase-firestore) | Firestore NoSQL DB                         | `0.1.x` |
| [`firebase-realtime-db`](https://crates.io/crates/firebase-realtime-db) | Realtime Database                          | `0.1.x` |
| [`firebase-messaging`](https://crates.io/crates/firebase-messaging) | Firebase Cloud Messaging (FCM)             | `0.1.x` |
| [`firebase-store`](https://crates.io/crates/firebase-store) | Firebase Storage                           | `0.1.x` |
| [`firebase-sdk`](https://crates.io/crates/firebase-sdk) | Unified SDK entry point (re-exports all)   | `0.1.x` |

---

## ✨ Features

- ✅ Fully asynchronous (uses `reqwest`/`tokio`)
- ✅ Modular design — use only what you need
- ✅ Type-safe Firestore & RTDB queries
- ✅ Simple token-based authentication
- ✅ Clean error handling via `thiserror`
- ✅ Ready for production and embedded systems

---

## 🚀 Getting Started

```toml
# In your Cargo.toml
firebase-sdk = "0.1"
```

Or pick only what you need:

```toml
firebase-authentication = "0.1"
firebase-firestore = "0.1"
```

---

## 🛠 Workspace Development

Build all crates:

```bash
cargo build --workspace
```

Run tests:

```bash
cargo test --workspace
```


---

## 🔐 License

Dual-licensed under **MIT** or **Apache-2.0** — your choice.  
© 2025 [Sayed Ali Sina Hussaini](mailto:s.alisinahussaini313@gmail.com)

---

## 🌍 Connect

- GitHub: [@real-all](https://github.com/real-all)
- Email: s.alisinahussaini313@gmail.com

---

> Rust + Firebase = ❤️ blazing-fast backend solutions.
