FROM rustlang/rust:nightly

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install \
    -y --no-install-recommends --no-install-suggests vim git

RUN rustup install stable

RUN git clone https://github.com/dusk-network/plonk.git
WORKDIR plonk
RUN cargo build --release
RUN cargo bench plonk