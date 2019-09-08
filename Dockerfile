FROM rust:stretch AS builder
COPY . /workdir
RUN cd /workdir && cargo build --release

FROM ubuntu:latest

COPY --from=builder /workdir/target/release/manga-crawler /manga-crawler

RUN apt-get update && apt-get install -y libsqlite3-dev libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

CMD HTTP_PORT=$PORT  /manga-crawler