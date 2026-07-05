FROM rust:1-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p agentready-server

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/agentready-server /usr/local/bin/agentready-server
ENV PORT=8080
EXPOSE 8080
CMD ["agentready-server"]
