FROM clux/muslrust:latest AS builder
WORKDIR /build
COPY Cargo.toml Cargo.lock /build/
COPY src /build/src/
RUN cargo build --release && cp $(find /build -xdev -name ifconfig-rs) /

FROM alpine:latest
WORKDIR /
COPY Rocket.toml /
COPY geoip /geoip
COPY templates /templates
COPY --from=builder /ifconfig-rs /
ENV ROCKET_PROFILE="release"
CMD ["/ifconfig-rs"]
