FROM lukemathwalker/cargo-chef:latest-rust-1@sha256:3a1dd6010466c1cf591607a75c6e144ba9f099e7f3790d595c6a611a9c78f387 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin axum-reverse-proxy

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim@sha256:60eac759739651111db372c07be67863818726f754804b8707c90979bda511df AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/opensearch-filter-proxy /usr/local/bin

ARG USER_ID=1001
ARG GROUP_ID=1001
RUN groupadd -g ${GROUP_ID} nonrootgroup && useradd -r -u ${USER_ID} -g nonrootgroup nonrootuser

USER nonrootuser

EXPOSE 3000

ENTRYPOINT ["/usr/local/bin/opensearch-filter-proxy"]
