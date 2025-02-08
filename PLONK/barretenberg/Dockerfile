FROM ubuntu:22.04 as base

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get update && apt-get install \
    -y build-essential cmake git libgmp3-dev libprocps-dev libboost-all-dev libssl-dev libsodium-dev nano curl ninja-build binaryen libomp-dev

RUN git clone https://github.com/AztecProtocol/barretenberg.git

WORKDIR barretenberg/cpp

# RUN chmod +x bootstrap.sh && ./bootstrap.sh

# RUN cmake --install build

RUN cmake --build --preset gcc --target ecc_tests