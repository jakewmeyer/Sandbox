FROM rust:1.72.0-alpine AS builder
RUN apk add musl-dev musl-utils
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:latest
COPY --from=builder /target/x86_64-unknown-linux-musl/release/sandbox-api .
ENTRYPOINT ["/sandbox-api"]
