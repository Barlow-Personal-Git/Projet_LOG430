FROM rust:1.82 AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY templates ./templates
COPY fonts ./fonts
COPY Rocket.toml ./Rocket.toml

RUN cargo build --release

FROM debian:stable-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates binutils && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/release/maison_mere /usr/local/bin/maison_mere

COPY --from=builder /usr/src/app/templates ./templates
COPY --from=builder /usr/src/app/fonts ./fonts
COPY --from=builder /usr/src/app/Rocket.toml ./Rocket.toml

RUN strip /usr/local/bin/maison_mere

EXPOSE 8000

CMD ["maison_mere"]
