FROM rust:1.93 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations
COPY scripts ./scripts
COPY .sqlx ./.sqlx

ENV SQLX_OFFLINE=true
RUN cargo install sqlx-cli --version "0.8.2" --no-default-features --features postgres
RUN cargo build --release

FROM rust:1.93-slim

WORKDIR /app

RUN apt-get update \
    && apt-get install -y ca-certificates bash curl \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rust_raw_server /app/rust_raw_server
COPY --from=builder /app/migrations /app/migrations
COPY --from=builder /app/scripts/docker-entrypoint.sh /app/docker-entrypoint.sh
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx

RUN chmod +x /app/docker-entrypoint.sh

EXPOSE 7878

CMD ["/app/docker-entrypoint.sh"]