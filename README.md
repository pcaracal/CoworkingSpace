# CoworkingSpace

## Projektbeschreibung (Deutsch)

Das Abschlussprojekt im M223 ÜK. Eine API die ermöglicht das Reservieren von Räumen.
Reservationen können von Mitgliedern angefragt werden und von Administratoren bestätigt oder abgelehnt werden.

# Setup

## Prerequisites

1. [Rust](https://rustup.rs/)
2. SQLite3
3. libsqlite3-sys (Debian Package: `librust-libsqlite3-sys-dev`)
4. [Diesel](https://diesel.rs/guides/getting-started): `cargo install diesel_cli --no-default-features --features sqlite`
5. [(Frontend) Latest Node.js](https://nodejs.org/en)
6. [(Frontend) Angular CLI v18](https://www.npmjs.com/package/@angular/cli)

## Run

### Backend

1. `cd backend`
2. `diesel setup`
3. Run:

- `cargo run --release` (Release, longer build time)
- `cargo run` (Debug, faster build time but slower performance)
- `cargo run (--release) -- test` (Delete all previous data and load test data)
- `cargo run (--release) -- clean` (Delete all previous data)

4. Swagger UI is available at `http://localhost:8000/swagger-ui/`

### Frontend

1. `cd frontend`
2. `npm i`
3. `ng serve (--open)`
