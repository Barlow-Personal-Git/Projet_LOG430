FROM rust:latest as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo build --release

FROM debian:stable-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends binutils libpq5 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/magasin_enligne /usr/local/bin/magasin_enligne

RUN strip /usr/local/bin/magasin_enligne

EXPOSE 8000

CMD ["magasin_enligne"]
