# Basic Site

An example website with a simple tech stack.

- Rust
- Axum for web server
- sqlx for database access
- askama for templating
- htmx for reactivity
- picocss for styling
- justfile for development recipes
- todo: docker deployment

## Features

- log in, log out, sign up (passwords stored in plaintext)
- multiple pages (home, profile)

## Setup

```shell
sqlx migrate add --source db/migrations init
```

### `.env` file

```shell
MIGRATIONS_PATH=db/migrations
DATABASE_PATH=db/db.db
DATABASE_URL=sqlite:${DATABASE_PATH}
```
