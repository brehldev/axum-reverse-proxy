FROM lukemathwalker/cargo-chef:latest-rust-1@sha256:7341ca3ca2726f3249fb227b510ce195fb80d6e584ff8894b68ffa97f1512e62 AS chef
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
FROM debian:bookworm-slim@sha256:7b140f374b289a7c2befc338f42ebe6441b7ea838a042bbd5acbfca6ec875818 AS runtime

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
