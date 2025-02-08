FROM ubuntu:22.04 as base

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get update && apt-get install \
    -y build-essential cmake git libgmp3-dev libprocps-dev libboost-all-dev libssl-dev libsodium-dev nano

RUN git clone https://github.com/scipr-lab/libiop.git

WORKDIR libiop

COPY fixes/CMakeLists.txt .
COPY fixes/utils.cpp libiop/algebra/utils.cpp
COPY fixes/utils.hpp libiop/algebra/utils.hpp
COPY test_matmul_snark.cpp /libiop/libiop/tests/snark/test_ligero_snark.cpp

RUN git config --global url.https://github.com:/.insteadOf git://github.com/

RUN git submodule update --init --recursive

RUN mkdir build && cd build && cmake .. && make

WORKDIR build

ADD "https://www.random.org/cgi-bin/randbyte?nbytes=10&format=h" skipcache

RUN ./libiop/test_ligero_snark