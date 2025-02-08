FROM ubuntu:22.04 as base

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get update && apt-get install \
    -y build-essential cmake git libgmp3-dev libprocps-dev libboost-all-dev libssl-dev libsodium-dev nano wget clang lld libomp-dev curl \
    bash python3-pip

ENV NODE_VERSION=16.13.0
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
ENV NVM_DIR=/root/.nvm
RUN . "$NVM_DIR/nvm.sh" && nvm install ${NODE_VERSION}
RUN . "$NVM_DIR/nvm.sh" && nvm use v${NODE_VERSION}
RUN . "$NVM_DIR/nvm.sh" && nvm alias default v${NODE_VERSION}
ENV PATH="/root/.nvm/versions/node/v${NODE_VERSION}/bin/:${PATH}"
RUN node --version
RUN npm --version

RUN npm install -g snarkjs@latest

RUN pip3 install git+https://github.com/meilof/pysnark

RUN pip3 install python-libsnark

RUN git clone https://github.com/meilof/pysnark.git

WORKDIR pysnark

RUN cd examples && PYSNARK_BACKEND=libsnarkgg python3 cube.py 3

COPY matmult.py matmult.py
COPY sha256.py sha256.py

RUN rm -f pysnark_*
RUN python3 matmult.py

RUN rm -f pysnark_*
RUN python3 sha256.py
