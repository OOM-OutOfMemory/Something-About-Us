FROM rust:alpine AS builder

WORKDIR /app
COPY . .
RUN apk add --no-cache libc-dev openssl-dev curl

# Rust statically links to libc, but alpine does not like it
# (especially in multistage build)
# See https://github.com/sfackler/rust-native-tls/issues/190
ENV RUSTFLAGS=-Ctarget-feature=-crt-static
RUN cargo build --release


FROM alpine:latest
RUN apk add --no-cache libgcc openssl ca-certificates curl
RUN mkdir /usr/local/bin/jwks

WORKDIR /usr/local/bin

COPY --from=builder /app/config.toml /usr/local/bin/config.toml
COPY --from=builder /app/target/release/something_about_us /usr/local/bin/something_about_us

CMD [ "/usr/local/bin/something_about_us" ]
