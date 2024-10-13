FROM rust:1.81.0-alpine AS base
RUN apk add musl-dev musl-utils
RUN cargo build --target x86_64-unknown-linux-musl --release --bin sandbox-api

FROM alpine AS runtime
RUN addgroup -S app && adduser -S app -G app
COPY --from=builder /target/x86_64-unknown-linux-musl/release/sandbox-api /usr/local/bin/
USER app
CMD ["/usr/local/bin/sandbox-api"]
