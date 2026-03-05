# va-paste

A high-performance Pastebin SaaS built with Rust, Axum, and PostgreSQL.

## Features
- Create pastes with optional titles and expiration times.
- Securely retrieve pastes by ID.
- Automatically handles expired pastes.
- Production-ready with Docker and CI/CD.

## Tech Stack
- **Web Framework:** [Axum](https://github.com/tokio-rs/axum)
- **Database:** [PostgreSQL](https://www.postgresql.org/)
- **ORM/Query Builder:** [SQLx](https://github.com/launchbadge/sqlx)
- **Runtime:** [Tokio](https://tokio.rs/)

## Setup

1. **Clone the repository**
2. **Setup Database**
   ```bash
   export DATABASE_URL=postgres://user:password@localhost/va_paste
   sqlx db create
   sqlx migrate run
   ```
3. **Run the application**
   ```bash
   cargo run
   ```

## Usage

### Create a Paste
```bash
curl -X POST http://localhost:3000/pastes \
  -H "Content-Type: application/json" \
  -d '{"content": "Hello World!", "title": "My First Paste"}'
```

### Get a Paste
```bash
curl http://localhost:3000/pastes/<uuid>
```
