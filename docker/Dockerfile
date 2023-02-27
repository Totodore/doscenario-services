FROM rust:1.65.0-slim AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/target/release/ .

CMD ["./app"]