FROM ubuntu:22.04 as base

RUN apt-get update && \
    apt-get install -y \
    wget unzip curl \
    build-essential cmake git libgmp3-dev libprocps-dev libboost-all-dev libssl-dev pkg-config python-pip openjdk-11-jdk

RUN apt-get install -y ant python3-pip && apt-get install ca-certificates-java

ENV JAVA_HOME /usr/lib/jvm/java-11-openjdk-amd64/
RUN export JAVA_HOME

RUN pip install markdown

RUN mkdir /mir
WORKDIR /mir
RUN git clone https://github.com/nojansheybani/mirage.git
WORKDIR /mir/mirage
RUN git config --global url.https://github.com:/.insteadOf git://github.com/
RUN git submodule update --init --recursive
RUN cd MirageBackend && sh integrate.sh

RUN cd libsnark && mkdir build && cd build && cmake .. && make run_universal_gg_ppzksnark

COPY SHA256Demo.java ./MirageCircuitGenerator/src/examples/

RUN cd MirageCircuitGenerator && mkdir -p bin && javac -d bin $(find ./src/* | grep ".java$")

ADD "https://www.random.org/cgi-bin/randbyte?nbytes=10&format=h" skipcache

RUN cd MirageCircuitGenerator && java -cp bin examples.SHA256Demo && cp sha256.in univ_circuit.arith /mir/mirage/libsnark/build/libsnark/mirage_interface

WORKDIR libsnark/build/libsnark/mirage_interface

RUN ./run_universal_gg_ppzksnark univ_circuit.arith sha256.in
