FROM ubuntu:18.04

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get update && apt-get install \
    -y --no-install-recommends --no-install-suggests vim git \ 
    cmake build-essential libssl-dev ca-certificates

RUN apt-get update
RUN apt-get install -y python3-dev python3-pip libssl-dev cmake
RUN pip3 install --upgrade pip

RUN pip3 install numpy==1.16.4 --user
RUN pip3 install tensorflow==1.14.0 --user

RUN git clone https://github.com/nickshey/Rosetta.git --recursive

RUN cd Rosetta && ./rosetta.sh compile --enable-protocol-zk --enable-tests && ./rosetta.sh install