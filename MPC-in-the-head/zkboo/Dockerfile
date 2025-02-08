FROM ubuntu:22.04 as base

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get update && apt-get install \
    -y build-essential cmake git libgmp3-dev libprocps-dev libboost-all-dev libssl-dev libsodium-dev nano

RUN git clone https://github.com/IAIK/gzkbpp.git

WORKDIR gzkbpp/code

COPY _compile_zkbpp.sh .

RUN chmod +x _compile_zkbpp.sh && ./_compile_zkbpp.sh