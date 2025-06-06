# 🔐 axum-app-auth

**axum-app-auth** is a clean, modular authentication API built with [Axum](https://github.com/tokio-rs/axum), leveraging Rust's performance and safety guarantees. Designed following **Clean Architecture** and **SOLID principles**, this project is ideal for scalable web services.

---
## ✨ Features

- ✅ Register & login endpoints
- 🔐 JWT-based authentication
- 🧼 Password hashing
- 🧠 Clean architecture with domain separation
- 🩺 Health check endpoint
- 🔌 PostgreSQL with SQLx
- 🚀 Production-ready and testable

---
## 📁 Folder Structure
```bash

└───axum-app-auth
    ├───migrations
    │   │
    ├───src
    │   ├───api
    │   │
    │   ├───config
    │   │
    │   ├───domain
    │   │
    │   ├───infrastructure
    │   │
    │   ├───middleware
    │   │
    │   ├───repository
    │   │
    │   └───service
```


## 🛠️ Getting Started

### 📦 Prerequisites

- Rust (stable)  
- PostgreSQL  
- [sqlx-cli](https://crates.io/crates/sqlx-cli) for database migrations  

### 🚀 Run Locally

```bash
# Clone the repository
git clone https://github.com/your-username/axum-app-auth.git
cd axum-app

# Set up .env file
cp .env 
# Run database migrations
sqlx migrate run

# Build & run the server
cargo run
