FROM ubuntu:22.04 as base

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get update && apt-get install \
    -y wget unzip curl --no-install-recommends --no-install-suggests vim git \ 
    cmake build-essential cmake git libgmp3-dev libprocps-dev libboost-all-dev libssl-dev pip pkg-config

RUN pip install markdown
    
RUN  git clone https://github.com/imdea-software/legosnark.git

WORKDIR legosnark
RUN git config --global url.https://github.com:/.insteadOf git://github.com/

RUN git submodule update --init --recursive

RUN mkdir -p build && cd build && cmake .. && cd depends && make -j8 && make -C libsnark install
 
WORKDIR src
COPY sha256.cc examples/sha256.cc
RUN printf "\n\n%s\n%s\n\n" "add_executable(sha256 sha256.cc)" "target_link_libraries(sha256 snark legobasic)" >> ./examples/CMakeLists.txt
WORKDIR ..

WORKDIR build/src
RUN make -j8 && ./examples/matrixsc 5
RUN make -j8 && ./examples/sha256

