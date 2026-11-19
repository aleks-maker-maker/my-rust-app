FROM rust:1.100.0-slim-trixie AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/
RUN cargo build --release --locked

FROM debian:trixie-slim
RUN useradd --create-home appuser
WORKDIR /home/appuser
COPY --from=builder /app/target/release/my-rust-app ./
USER appuser
CMD ["./my-rust-app"]
