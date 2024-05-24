# https://hub.docker.com/_/rust
FROM rust:1.63-alpine as build

WORKDIR /build
RUN apk add --no-cache -u \
    bash \
    build-base \
    curl \
    gcc \
    git \
    musl-dev \
    openssl-dev \
    pkgconfig
RUN curl -sSf https://just.systems/install.sh | bash -s -- --to /usr/local/bin
ADD ./ .
RUN just build

# FROM scratch
# https://distroless.dev
FROM distroless.dev/alpine-base as release

# COPY --from=build /build/target/release/did-web-server /did-web-server
COPY --from=build /build/target/debug/did-web-server /did-web-server

# Rocket configuration settings: https://rocket.rs/v0.5-rc/guide/configuration/
ENV ROCKET_PORT="3000"
ENV ROCKET_ADDRESS="0.0.0.0"
ENV ROCKET_LOG_LEVEL="normal"
# ENV ROCKET_LOG_LEVEL="debug"
ENV ROCKET_IDENT=false

EXPOSE 3000

ENTRYPOINT ["/did-web-server"]
