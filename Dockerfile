FROM rust:1.79.0-alpine3.20 AS build_base

RUN apk --no-cache upgrade && \
    apk add --no-cache cargo && \
    rm -rf /tmp/*

FROM build_base AS builder

WORKDIR /build

COPY Cargo.toml ./
COPY src ./src

ENV RUSTFLAGS="-C target-feature=-crt-static"

RUN cargo build --release

RUN strip target/release/marytts-pico

FROM alpine:3.20 AS run_base

RUN apk --no-cache upgrade && \
    apk add --no-cache dumb-init picotts libgcc && \
    rm -rf /tmp/*

FROM run_base

WORKDIR /app

COPY --from=builder /build/target/release/marytts-pico ./

ENTRYPOINT ["dumb-init", "/app/marytts-pico"]