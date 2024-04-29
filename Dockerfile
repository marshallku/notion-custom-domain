FROM rust:1.73-alpine AS base

WORKDIR /usr/src/notion-custom-domain

RUN set -eux; \
    apk add --no-cache musl-dev pkgconfig libressl-dev; \
    rm -rf $CARGO_HOME/registry

COPY Cargo.* .

RUN mkdir src && \
    echo 'fn main() {println!("Hello, world!");}' > src/main.rs && \
    cargo build --release && \
    rm target/release/notion-custom-domain* && \
    rm target/release/deps/notion_custom_domain* && \
    rm -rf src

FROM base AS builder

COPY src src
RUN cargo build --release

FROM alpine:3.14

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/notion-custom-domain/target/release/notion-custom-domain .

EXPOSE ${PORT}

CMD ["./notion-custom-domain"]