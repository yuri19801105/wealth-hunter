FROM rust:1.75-slim as builder

WORKDIR /app

COPY backend /app/backend
COPY frontend /app/frontend

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app/backend

RUN cargo build --release

WORKDIR /app/frontend

RUN cargo install wasm-pack && \
    wasm-pack build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/backend/target/release/wealth_hunter /app/
COPY --from=builder /app/backend/target/release/audit.exe /app/
COPY --from=builder /app/backend/target/release/diagnose_engine.exe /app/
COPY --from=builder /app/backend/target/release/test_ooda_act.exe /app/
COPY --from=builder /app/backend/target/release/verify_performance.exe /app/
COPY --from=builder /app/backend/target/release/verify_ooda_refactor.exe /app/
COPY --from=builder /app/frontend/pkg /app/frontend

RUN chmod +x /app/wealth_hunter

EXPOSE 8080

CMD ["/app/wealth_hunter"]
