FROM ubuntu:22.04 as base

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get update && apt-get install \
    -y build-essential cmake git libgmp3-dev libprocps-dev libboost-all-dev libssl-dev libsodium-dev nano curl

RUN git clone https://github.com/mir-protocol/plonky2.git
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup default nightly

WORKDIR plonky2/plonky2

RUN cargo clean
RUN cargo build
RUN cargo bench merkle