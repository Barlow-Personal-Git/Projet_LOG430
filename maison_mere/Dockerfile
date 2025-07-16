FROM rust:latest as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

FROM debian:stable-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends binutils ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/maison_mere /usr/local/bin/maison_mere

RUN strip /usr/local/bin/maison_mere

EXPOSE 8000

CMD ["maison_mere"]
