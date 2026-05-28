FROM rust:1.95-bookworm AS builder
WORKDIR /app

COPY . .
RUN cargo build --release --bins --locked

FROM debian:bookworm-slim AS runner
RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app
ENV RUST_LOG=info

COPY --from=builder /app/target/release/bidmart-admin-be /usr/local/bin/app

EXPOSE 8080
CMD ["/usr/local/bin/app"]
