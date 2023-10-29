FROM ubuntu:23.10

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        libasound2-dev \
        libudev-dev \
        pkg-config \
        build-essential \
        curl \
        ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 --retry 10 --retry-connrefused --location --silent --show-error --fail "https://sh.rustup.rs" | sh -s -- --default-toolchain stable -y
