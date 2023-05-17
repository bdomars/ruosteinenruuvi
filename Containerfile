FROM rust:latest AS builder

COPY src/ /build/src/
COPY Cargo.toml /build
COPY Cargo.lock /build

WORKDIR /build
RUN apt update && apt-get install --assume-yes libdbus-1-dev pkg-config libssl-dev
RUN cargo build --release

FROM ubuntu:latest AS runtime

RUN apt update && apt-get install --assume-yes libdbus-1-3

WORKDIR /app
COPY --from=builder /build/target/release/ruuvi /app/ruuvi
COPY Tags.toml /app

ENTRYPOINT /app/ruuvi
