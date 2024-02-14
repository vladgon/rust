FROM rust:1 as chef
RUN cargo install cargo-chef &&  \
    apt update &&  \
    apt -y install protobuf-compiler
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo install --path wg_grpc_tonic

FROM debian:bookworm-slim AS runtime
#RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
ENV grpc_host=[::0]
COPY wg_sample_app/resources /resources
COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server
COPY --from=builder /usr/local/cargo/bin/client /usr/local/bin/client