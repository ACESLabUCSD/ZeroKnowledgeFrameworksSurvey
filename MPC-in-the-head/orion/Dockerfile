FROM debian:testing

RUN apt-get update && \
    apt-get install -y \
    wget unzip curl \
    cmake git libprocps-dev libboost-all-dev libssl-dev pkg-config pip make clang++-7 libgmp-dev g++ parallel python3

# RUN update-alternatives --install /usr/bin/clang++ clang++ /usr/bin/clang++-7 1000

RUN git clone https://github.com/niconiconi/OrionLinearPC.git

WORKDIR OrionLinearPC

RUN git submodule update --init --recursive

RUN cmake . && make -j4