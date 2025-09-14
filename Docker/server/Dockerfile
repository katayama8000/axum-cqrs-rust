FROM rust:1.86.0-bookworm

ENV CARGO_BUILD_TARGET_DIR=/tmp/target
RUN apt-get update && apt-get install -y redis-tools && rm -rf /var/lib/apt/lists/*
RUN rustup component add rustfmt clippy
# Error occurs when installing cargo-watch So I commented out
# RUN cargo install cargo-watch
RUN cargo install cargo-nextest --locked