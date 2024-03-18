# Basic Site

An example website with a simple tech stack.

- Rust
- Axum for web server
- sqlx for database access
- askama for templating
- htmx for reactivity
- picocss for styling
- justfile for development recipes
- docker for deployment

## Features

- log in, log out, sign up
- multiple pages (home, profile)

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

## Further reading

- https://joeymckenzie.tech/blog/templates-with-rust-axum-htmx-askama
- https://www.bitswired.com/en/blog/post/rustgpt-journey-rust-htmx-web-dev