FROM ubuntu:22.04 as base

RUN apt-get update && \
    apt-get install -y \
    wget unzip curl \
    build-essential cmake git libgmp3-dev libprocps-dev libboost-all-dev libssl-dev pkg-config pip

RUN pip install markdown

RUN mkdir /zcoinn-libsnark
WORKDIR /zcoinn-libsnark
RUN git init
RUN git config --global url.https://github.com:/.insteadOf git://github.com/
RUN mkdir depends && cd depends && \
    echo "add_subdirectory(libsnark)" > CMakeLists.txt && \
    git submodule add https://github.com/scipr-lab/libsnark.git libsnark
COPY CMakeLists.txt CMakeLists.txt

RUN git submodule update --init --recursive

COPY src/ src/

RUN mkdir build && cd build && cmake .. && make

WORKDIR build/src
