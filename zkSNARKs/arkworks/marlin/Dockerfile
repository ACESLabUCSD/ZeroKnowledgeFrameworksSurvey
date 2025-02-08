FROM rustlang/rust:nightly

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install \
    -y --no-install-recommends --no-install-suggests vim git

RUN rustup install stable

# COPY matmul/ matmul/
# WORKDIR matmul
# RUN cargo run --release
# WORKDIR ..

COPY sha256/ sha256/
WORKDIR sha256
RUN cargo run --release
WORKDIR ..