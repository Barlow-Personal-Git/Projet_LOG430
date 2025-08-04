FROM rust:1.82 AS builder

RUN apt-get update && apt-get install -y libpq-dev pkg-config build-essential

WORKDIR /usr/src/app

COPY Cargo.toml ./
COPY src ./src

RUN cargo install diesel_cli --no-default-features --features postgres

RUN cargo build --release

COPY . .
RUN cargo install --path . --locked

FROM debian:stable-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends libpq5 ca-certificates binutils && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/magasin /usr/local/bin/magasin
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

RUN strip /usr/local/bin/magasin

EXPOSE 8000

CMD ["magasin"]
