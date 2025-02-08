FROM ubuntu:22.04 as base

RUN apt-get update && DEBIAN_FRONTEND=noninteractive \
    apt-get install -y \
    wget unzip curl \
    build-essential cmake git libgmp3-dev libprocps-dev pkg-config python3-pip \
    g++ libboost-all-dev libssl-dev libjsoncpp-dev libgtest-dev openjdk-8-jdk maven javacc ca-certificates-java ant

ENV JAVA_HOME /usr/lib/jvm/java-8-openjdk-amd64/
RUN export JAVA_HOME

RUN git clone https://github.com/TrustworthyComputing/Zilch
WORKDIR /Zilch
RUN git clone https://github.com/TrustworthyComputing/ZeroJava-compiler.git

RUN cd ZeroJava-compiler && mvn initialize && mvn package

RUN make -j8 && make zilch-tests -j8

RUN ./zilch-tests
# RUN ./zilch --asm ./examples-zmips/read_test/read_test.zmips --tsteps 5 --pubtape ./examples-zmips/read_test/read_test.pubtape --auxtape ./examples-zmips/read_test/read_test.auxtape

WORKDIR /Zilch/ZeroJava-compiler
RUN java -jar target/zerojava-compiler-1.0.jar src/test/resources/Add.java