FROM rust:1.91-slim AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY common/ common/
COPY server/ server/
COPY cli/ cli/
COPY migrations/ migrations/

RUN cargo build --release -p epoch-server

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/epoch-server /usr/local/bin/epoch-server
COPY --from=builder /app/migrations /migrations

ENV PORT=3000
EXPOSE 3000

CMD ["epoch-server"]
