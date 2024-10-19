FROM rust:1.82-slim AS base
RUN apt-get update && apt-get install -y mold
RUN cargo install cargo-chef --locked

FROM base AS chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=chef /recipe.json recipe.json
RUN cargo chef cook --target x86_64-unknown-linux-gnu --release  --bin sandbox-api --recipe-path recipe.json
COPY . .
RUN cargo build --target x86_64-unknown-linux-gnu --release --bin sandbox-api

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /target/x86_64-unknown-linux-gnu/release/sandbox-api /usr/local/bin/
CMD ["/usr/local/bin/sandbox-api"]
