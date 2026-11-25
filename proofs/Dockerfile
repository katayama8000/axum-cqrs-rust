FROM ubuntu:24.04

ARG TARGETARCH
ARG AENEAS_RELEASE=nightly-2026.11.24-fb44cb4
ENV PATH="/opt/lean/bin:/opt/aeneas:/root/.cargo/bin:$PATH"
ENV RUSTUP_TOOLCHAIN=nightly-2026-09-17

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates curl gcc git zstd \
    && rm -rf /var/lib/apt/lists/*

COPY proofs/lean-toolchain /tmp/lean-toolchain
RUN set -eu; \
    case "$TARGETARCH" in \
      arm64) aeneas_arch=aarch64; lean_arch=linux_aarch64 ;; \
      amd64) aeneas_arch=x86_64; lean_arch=linux ;; \
      *) exit 1 ;; \
    esac; \
    lean_version=$(sed 's|leanprover/lean4:v||' /tmp/lean-toolchain); \
    mkdir /opt/lean /opt/aeneas; \
    curl -fsSL --retry 3 -o /tmp/lean.tar.zst \
      "https://github.com/leanprover/lean4/releases/download/v${lean_version}/lean-${lean_version}-${lean_arch}.tar.zst"; \
    tar --zstd -xf /tmp/lean.tar.zst --strip-components=1 -C /opt/lean; \
    rm /tmp/lean.tar.zst; \
    curl -fsSL --retry 3 -o /tmp/aeneas.tar.gz \
      "https://github.com/AeneasVerif/aeneas/releases/download/${AENEAS_RELEASE}/aeneas-linux-${aeneas_arch}.tar.gz"; \
    tar -xzf /tmp/aeneas.tar.gz -C /opt/aeneas; \
    rm /tmp/aeneas.tar.gz

RUN curl -fsSL --retry 3 -o /tmp/rustup.sh https://sh.rustup.rs \
    && sh /tmp/rustup.sh -y --profile minimal --default-toolchain none \
    && rm /tmp/rustup.sh \
    && rustup toolchain install "$RUSTUP_TOOLCHAIN" --profile minimal --component rustc-dev,rust-src

WORKDIR /opt/aeneas/backends/lean
# The release includes Aeneas oleans. Fetch the dependency modules they import.
RUN modules=$(grep -hrE '(^| )import (Mathlib|Batteries)(\.|$)' Aeneas AeneasMeta ./*.lean \
      | sed -E 's/^.*import ([^ ]+).*$/\1/' | sort -u) \
    && test -n "$modules" \
    && lake exe cache get $modules

WORKDIR /work
