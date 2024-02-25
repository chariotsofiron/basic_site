set dotenv-load

init:
  cargo install cargo-watch
  cargo install sqlx-cli
  sqlx database create
  just db-migrate

db-migrate:
  echo "Migrating..."
  sqlx migrate run --source $MIGRATIONS_PATH;

db-reset:
  echo "Resetting..."
  sqlx database drop && sqlx database create && sqlx migrate run --source $MIGRATIONS_PATH
  sqlite3 $DATABASE_PATH < seeds/seed.sql

build-server:
  cargo build --release

dev-server:
	cargo watch --no-dot-ignores -w src -w templates -x run
