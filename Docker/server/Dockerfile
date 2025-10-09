FROM rust:1.86.0-bookworm

ENV CARGO_BUILD_TARGET_DIR=/tmp/target
RUN rustup component add rustfmt clippy
RUN cargo install bacon --locked
RUN cargo install cargo-nextest --locked
RUN apt-get update && apt-get install -y redis-tools default-mysql-client && rm -rf /var/lib/apt/lists/*