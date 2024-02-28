# Stage 1: Building the application
FROM rust:1.76.0 as builder

# Create a new empty shell project
RUN USER=root cargo new --bin basic_site
WORKDIR /basic_site

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# This trick will cache our dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy our source code
COPY ./src ./src
COPY ./db/migrations ./db/migrations
COPY ./templates ./templates

# Set environment variables required for build and runtime
ENV MIGRATIONS_PATH=db/migrations
ENV DATABASE_PATH=db/db.db
ENV DATABASE_URL=sqlite:${DATABASE_PATH}

RUN cargo install sqlx-cli
RUN sqlx database create
RUN sqlx migrate run --source $MIGRATIONS_PATH
RUN rm ./target/release/deps/basic_site*
RUN cargo build --release

# Stage 2: Preparing the final image
FROM debian:bookworm-slim

COPY --from=builder /basic_site/target/release/basic_site .
COPY --from=builder /basic_site/db ./db

ENV MIGRATIONS_PATH=db/migrations
ENV DATABASE_PATH=db/db.db
ENV DATABASE_URL=sqlite:${DATABASE_PATH}

CMD ["./basic_site"]