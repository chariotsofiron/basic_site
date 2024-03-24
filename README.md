# Basic Site

An example website with a simple tech stack and architecture.

- Rust
- Axum for web server
- sqlx for database access
- askama for templating
- htmx for reactivity
- picocss for styling
- justfile for development recipes
- docker for deployment

## Features

- [x] log in, log out, sign up
- [x] password hashing with Argon2id
- [x] profile page with active sessions and delete them
- [ ] change username / password

## Run locally

```shell
sqlx migrate add --source db/migrations init
cargo run --release
```

## Deploy

```shell
sqlx migrate add --source db/migrations init

docker run -v ./db:/db basic_site
```

### `.env` file

```shell
MIGRATIONS_PATH=db/migrations
DATABASE_PATH=db/db.db
DATABASE_URL=sqlite:${DATABASE_PATH}
```
