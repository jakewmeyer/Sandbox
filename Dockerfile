FROM rust:1.72.0-alpine AS chef
RUN apk add musl-dev musl-utils
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=chef /recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:latest
COPY --from=builder /target/x86_64-unknown-linux-musl/release/sandbox-api .
ENTRYPOINT ["/sandbox-api"]
