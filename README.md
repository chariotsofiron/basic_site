# Website

An example website with a simple tech stack.

- Rust
- Axum for web server
- sqlx for database access
- askama for templating
- htmx for reactivity
- picocss for styling
- docker for deployment
- justfile for development recipes

## Features

- sign up
- sign in
- navigate pages
- view sessions
- delete sessions
- profile page

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
