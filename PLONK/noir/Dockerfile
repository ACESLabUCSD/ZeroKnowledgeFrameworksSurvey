FROM ubuntu:22.04 as base

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get update && apt-get install \
    -y build-essential cmake git libgmp3-dev libprocps-dev libboost-all-dev libssl-dev libsodium-dev nano wget clang lld libomp-dev curl bash xxd

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN git clone https://github.com/noir-lang/noir.git

WORKDIR noir

RUN git checkout tags/v0.3.2

RUN cargo install --locked --path=crates/nargo --no-default-features --features plonk_bn254

# RUN curl -L https://raw.githubusercontent.com/noir-lang/noirup/main/install | bash

# RUN cd /root/.nargo/bin/ && rm noirup && wget https://raw.githubusercontent.com/noir-lang/noirup/main/noirup && chmod +x noirup

# ENV PATH="${PATH}:/root/.nargo/bin"

# RUN noirup

RUN nargo --version

# If you want to avoid downloading fresh Ignite SRS everytime, download it from http://aztec-ignition.s3.amazonaws.com/MAIN%20IGNITION/sealed/transcript00.dat and inject into container.
# COPY transcript00.dat /root/noir_cache/ignition/transcript00.dat

COPY hash/sha256 sha256/

COPY matmult/ matmult/

WORKDIR sha256

RUN ./run.py

WORKDIR ../matmult

RUN ./run.py
