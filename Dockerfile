FROM rust:latest

RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
        libclang-dev

WORKDIR /surrealdb-docker

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

RUN cargo install --path .

CMD ["surrealdb-docker"]
