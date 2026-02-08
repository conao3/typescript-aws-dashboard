FROM rust:1.83 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 libpq5 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/backend /app/backend
COPY schema.sql /app/schema.sql

ENV RUST_LOG=info
ENV DATABASE_URL=postgres://dashboard:dashboard@postgres:5432/dashboard

EXPOSE 17231

CMD ["/app/backend"]
