FROM clux/muslrust:latest AS builder
WORKDIR /build
COPY Cargo.toml Cargo.lock /build/
COPY src /build/src/
RUN cargo build --release && cp $(find /build -xdev -name ifconfig-rs) /

FROM ghcr.io/lukaspustina/ifconfig-rs-data:latest AS data

FROM alpine:latest
WORKDIR /ifconfig-rs
COPY Rocket.toml .
COPY templates templates/
COPY --from=builder /ifconfig-rs .
COPY --from=data /data data/
CMD ["ifconfig-rs"]
