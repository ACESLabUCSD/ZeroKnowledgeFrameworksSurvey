FROM rustlang/rust:nightly

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install \
    -y --no-install-recommends --no-install-suggests vim git

RUN rustup install stable

RUN git clone https://github.com/matter-labs/bellman.git
WORKDIR bellman
RUN cargo build --release