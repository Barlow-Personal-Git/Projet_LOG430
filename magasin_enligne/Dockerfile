FROM rust:1.82 AS builder

RUN apt-get update && apt-get install -y libpq-dev pkg-config build-essential

WORKDIR /usr/src/app

COPY Cargo.toml ./ 
COPY src ./src
COPY migrations ./migrations
COPY templates ./templates

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo build --release
RUN cargo install --path .

FROM debian:stable-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends libpq5 ca-certificates binutils && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/magasin_enligne /usr/local/bin/magasin_enligne
COPY --from=builder /usr/src/app/templates /usr/src/app/templates
COPY Rocket.toml /usr/src/app/Rocket.toml

WORKDIR /usr/src/app

RUN strip /usr/local/bin/magasin_enligne

EXPOSE 8000

CMD ["magasin_enligne"]
