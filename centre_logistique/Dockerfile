FROM rust:latest as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo build --release

FROM debian:stable-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends binutils libpq5 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/centre_logistique /usr/local/bin/centre_logistique

RUN strip /usr/local/bin/centre_logistique

EXPOSE 8000

CMD ["centre_logistique"]
