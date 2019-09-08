FROM rust:stretch AS builder

COPY . /workdir

RUN cd /workdir && cargo build --release

FROM ubuntu:latest

COPY --from=builder /workdir/target/release/manga-crawler /manga-crawler

CMD ["/manga-crawler"]