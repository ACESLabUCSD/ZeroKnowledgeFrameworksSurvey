FROM rustlang/rust:nightly

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install \
    -y --no-install-recommends --no-install-suggests vim git libexpat1-dev libfreetype6-dev libcairo2-dev cmake

RUN rustup install stable

RUN git clone https://github.com/zcash/halo2.git
COPY rust-toolchain.toml /halo2/rust-toolchain.toml
WORKDIR halo2/halo2_gadgets
# COPY matmul.rs benches/matmul.rs
COPY sha256.rs benches/sha256.rs
# COPY Cargo.toml Cargo.toml
RUN cargo build --release
# RUN cargo add --git https://github.com/zkonduit/ezkl.git
# RUN cargo bench matmul
RUN cargo bench sha256