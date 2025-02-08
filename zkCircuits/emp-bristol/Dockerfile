FROM debian:testing

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get update && apt-get install \
    -y --no-install-recommends --no-install-suggests vim git \ 
    cmake build-essential libssl-dev ca-certificates

RUN apt-get install -y python3
    
RUN git clone --recursive https://github.com/emp-toolkit/emp-tool.git

WORKDIR emp-tool

COPY genCircuits.cpp test/genCircuits.cpp

COPY CMakeLists.txt test/CMakeLists.txt

RUN cmake . && make && make install 

RUN ./bin/test_genCircuits