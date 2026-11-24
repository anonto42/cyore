# syntax=docker/dockerfile:1
# Stage 1: build a fully static (musl) release binary. Edition 2024 needs Rust >= 1.85.
ARG RUST_VERSION=1.91
FROM rust:${RUST_VERSION}-alpine AS builder

# Workspace member to build (one crate per learning milestone)
ARG CRATE=xor

RUN apk add --no-cache musl-dev

WORKDIR /src
COPY . .

RUN cargo build --release --locked -p ${CRATE} && \
    cp target/release/${CRATE} /app

# Stage 2: empty runtime image — the binary is self-contained (Stage 4 goal: one tiny file)
FROM scratch

COPY --from=builder /app /app

USER 10001:10001

ENTRYPOINT ["/app"]
