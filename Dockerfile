FROM rust:1.75-slim-bookworm as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/va-paste /app/va-paste
ENV RUST_LOG=info
EXPOSE 3000
CMD ["./va-paste"]
