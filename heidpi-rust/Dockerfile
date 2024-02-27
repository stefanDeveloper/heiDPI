FROM rust:1.76.0 as builder
WORKDIR /usr/src/heidpi-rust
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update & apt-get install -y extra-runtime-dependencies glibc & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/heidpi /usr/local/bin/heidpi

CMD ["heidpi", "start", "--host",  "127.0.0.1", "--port", "7000"]