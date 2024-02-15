FROM rust:1.76.0 as builder
WORKDIR /usr/src/heidpi-rust
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update & apt-get install -y extra-runtime-dependencies glibc & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/heidpi-rust /usr/local/bin/heidpi-rust

CMD ["heidpi-rust"]