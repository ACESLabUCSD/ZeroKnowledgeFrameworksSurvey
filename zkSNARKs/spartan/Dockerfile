FROM rustlang/rust:nightly

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install \
    -y --no-install-recommends --no-install-suggests vim git

RUN rustup default nightly

RUN git clone https://github.com/microsoft/Spartan.git

WORKDIR Spartan

RUN cargo doc
RUN RUSTFLAGS="-C target_cpu=native" cargo test
RUN RUSTFLAGS="-C target_cpu=native" cargo build --release