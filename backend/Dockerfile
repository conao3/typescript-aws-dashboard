FROM rust:1.83 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 libpq5 postgresql-client wget && \
    wget -O /tmp/psqldef.tar.gz https://github.com/sqldef/sqldef/releases/download/v0.17.24/psqldef_linux_amd64.tar.gz && \
    tar -xzf /tmp/psqldef.tar.gz -C /usr/local/bin && \
    chmod +x /usr/local/bin/psqldef && \
    rm /tmp/psqldef.tar.gz && \
    apt-get remove -y wget && \
    apt-get autoremove -y && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/backend /app/backend
COPY schema.sql /app/schema.sql
COPY docker-entrypoint.sh /app/docker-entrypoint.sh

RUN chmod +x /app/docker-entrypoint.sh

ENV RUST_LOG=info
ENV DATABASE_URL=postgres://dashboard:dashboard@postgres:5432/dashboard
ENV POSTGRES_PASSWORD=dashboard

EXPOSE 17231

ENTRYPOINT ["/app/docker-entrypoint.sh"]
