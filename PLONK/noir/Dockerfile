FROM ubuntu:24.04 AS base

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get update && apt-get install \
    -y build-essential cmake git nano wget curl bash xxd jq python3


RUN curl -L https://raw.githubusercontent.com/noir-lang/noirup/main/install | bash

ENV PATH="$PATH:/root/.nargo/bin"

RUN noirup -v 1.0.0-beta.6

RUN nargo --version

RUN curl -L https://raw.githubusercontent.com/AztecProtocol/aztec-packages/refs/heads/master/barretenberg/bbup/install | bash

ENV PATH="$PATH:/root/.bb"

RUN bbup -v 0.87.0

# If you want to avoid downloading fresh Ignite SRS everytime, download it from http://aztec-ignition.s3.amazonaws.com/MAIN%20IGNITION/sealed/transcript00.dat and inject into container.
# COPY transcript00.dat /root/noir_cache/ignition/transcript00.dat

COPY hash/sha256 sha256/

COPY matmult/ matmult/

WORKDIR sha256

RUN ./run.py

WORKDIR ../matmult

RUN ./run.py
