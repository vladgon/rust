FROM rust:1 as chef
RUN cargo install cargo-chef &&  \
    apt update &&  \
    apt -y install protobuf-compiler && \
    apt -y install  libmariadbclient-dev-compat
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo install --path db_diesel

FROM debian:bookworm-slim AS runtime
RUN apt update  && \
    apt -y install libmariadbclient-dev-compat
#    && apt install -y extra-runtime-dependencies  && \
#    && rm -rf /var/lib/apt/lists/*
ENV grpc_host=[::0]
COPY wg_sample_app/resources /resources
COPY --from=builder /usr/local/cargo/bin/show_posts /usr/local/bin/show_posts