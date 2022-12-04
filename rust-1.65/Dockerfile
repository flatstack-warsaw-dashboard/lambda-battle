FROM rust:1.65.0-alpine

RUN rustup target add x86_64-unknown-linux-musl

RUN apk add --update \
    musl-dev \
    && rm -rf /var/cache/apk/*

WORKDIR /src
