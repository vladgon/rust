FROM rust:1.76-slim-bookworm as builder

RUN apt update && apt -y install protobuf-compiler

WORKDIR /usr/src/myapp

COPY . .
COPY wg_sample_app/resources /resources

RUN cargo install --path wg_grpc_tonic

FROM debian:bookworm-slim
#RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
ENV grpc_host=[::0]
COPY --from=builder /resources resources
COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server
COPY --from=builder /usr/local/cargo/bin/client /usr/local/bin/client