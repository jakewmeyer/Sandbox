FROM rust:1.73.0-alpine AS base
RUN apk add musl-dev musl-utils
RUN cargo install cargo-chef

FROM base AS chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=chef /recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release --bin sandbox-api

FROM alpine AS runtime
RUN addgroup -S app && adduser -S app -G app
COPY --from=builder /target/x86_64-unknown-linux-musl/release/sandbox-api /usr/local/bin/
USER app
CMD ["/usr/local/bin/sandbox-api"]
