FROM ubuntu:22.04 as base

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get update && apt-get install \
    -y build-essential cmake git libgmp3-dev libprocps-dev libboost-all-dev libssl-dev libsodium-dev nano curl

RUN git clone https://github.com/0xPolygonMiden/miden-vm.git
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR miden-vm

RUN cd miden && cargo build --profile optimized --features concurrent,executable

COPY matmul/ matmul/
COPY sha256/ sha256/
COPY fib/ fib/

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get update && apt-get install -y time

# SHA256 is deprecated on last Miden update
# RUN PROVE_OUTPUT=$(time ./target/optimized/miden prove -a sha256/sha256.masm -n 8 2>&1)
# RUN PROGRAM_HASH=$(echo "$PROVE_OUTPUT" | grep -oP 'Program hash: \K0x[0-9a-f]{64}')
# RUN time ./target/optimized/miden verify -p sha256/sha256.proof -h ${PROGRAM_HASH}

RUN PROVE_MATMUL_OUTPUT=$(time ./target/optimized/miden prove -a matmul/matmul.masm -n 1 2>&1)
RUN MATMUL_PROGRAM_HASH=$(echo "$PROVE_MATMUL_OUTPUT" | grep -oP 'Program hash: \K0x[0-9a-f]{64}')
RUN time ./target/optimized/miden verify -p matmul/matmul.proof -h ${MATMUL_PROGRAM_HASH}