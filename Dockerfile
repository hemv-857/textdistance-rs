# Port Mortem — textdistance Python → Rust
# One command build: docker build -t textdistance-rs . && docker run textdistance-rs

FROM rust:1.77-slim as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    python3 \
    python3-pip \
    && rm -rf /var/lib/apt/lists/*

RUN pip3 install textdistance

COPY --from=builder /app/target/release/textdistance /usr/local/bin/textdistance
COPY --from=builder /app/scripts /app/scripts
COPY --from=builder /app/fuzz /app/fuzz
COPY --from=builder /app/adapter /app/adapter
COPY --from=builder /app/tests /app/tests

WORKDIR /app

# Default: run the test parity verification
CMD ["bash", "scripts/verify_test_parity.sh"]
