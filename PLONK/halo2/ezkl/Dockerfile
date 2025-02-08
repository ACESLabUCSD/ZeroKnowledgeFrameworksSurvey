FROM rustlang/rust:nightly

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install \
    -y --no-install-recommends --no-install-suggests vim git libexpat1-dev libfreetype6-dev libcairo2-dev cmake

RUN rustup install stable

RUN git clone https://github.com/zkonduit/ezkl.git
WORKDIR ezkl
RUN cargo build --release
RUN cargo bench