FROM rustlang/rust:nightly

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install \
    -y --no-install-recommends --no-install-suggests vim git

# RUN git clone https://github.com/nojansheybani/swanky.git
COPY swanky ./swanky
WORKDIR swanky/diet-mac-and-cheese
RUN cargo build --release

COPY matmul ./matmul
COPY sha256 ./sha256
COPY config.toml ./

# RUN cargo run --bin dietmc -- --text --relation matmul/picozk_test.rel --instance matmul/picozk_test.type0.ins --connection-addr 127.0.0.1:7876 --witness matmul/picozk_test.type0.wit