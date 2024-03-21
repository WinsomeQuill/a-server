FROM rust:alpine as builder

WORKDIR /app/src

RUN USER=root

RUN apk add pkgconfig openssl-dev libc-dev

COPY src src
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
COPY .env .env

RUN cargo build --release

FROM alpine:latest

WORKDIR /app

RUN apk update \
    && apk add openssl ca-certificates

EXPOSE 9985

COPY --from=builder /app/src/target/release/a-server /app/a-server
COPY --from=builder /app/src/.env /app/.env

CMD ["/app/a-server"]