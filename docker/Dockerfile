FROM ubuntu:18.04

RUN apt-get update && apt-get install -y curl valgrind build-essential clang
# Install Rust
ENV RUST_VERSION=stable
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain=$RUST_VERSION