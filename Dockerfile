FROM rust:bookworm as builder
WORKDIR /usr/src/rustpaste
COPY . .
RUN cargo install --path .
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/rustpaste /usr/local/bin/rustpaste
CMD ["rustpaste"]
