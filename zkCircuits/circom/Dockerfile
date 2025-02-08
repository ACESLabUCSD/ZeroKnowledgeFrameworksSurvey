FROM rustlang/rust:nightly

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get update && apt-get install \
    -y build-essential cmake git libgmp3-dev libprocps-dev libboost-all-dev libssl-dev nano wget curl

ENV NODE_VERSION=18.18.0
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
ENV NVM_DIR=/root/.nvm
RUN . "$NVM_DIR/nvm.sh" && nvm install ${NODE_VERSION}
RUN . "$NVM_DIR/nvm.sh" && nvm use v${NODE_VERSION}
RUN . "$NVM_DIR/nvm.sh" && nvm alias default v${NODE_VERSION}
ENV PATH="/root/.nvm/versions/node/v${NODE_VERSION}/bin/:${PATH}"
RUN node --version
RUN npm --version
RUN npm install -g npm

RUN git clone https://github.com/iden3/circom.git

RUN git clone https://github.com/iden3/circomlib.git

WORKDIR circom

RUN cargo build --release && cargo install --path circom

RUN npm install -g snarkjs && mkdir src

# Select demo example by editing the path ("run_*.sh" can automatically do this)
COPY sha256/src/ ./src

WORKDIR src

RUN mkdir results && circom example.circom --r1cs --wasm --sym --c -o ./results

RUN apt-get install -y nlohmann-json3-dev nasm libgmp-dev

WORKDIR results/example_cpp

RUN cp ../../input.json ./input.json && cp ../example.r1cs ./example.r1cs

RUN make && ./example ./input.json ./witness.wtns

RUN snarkjs powersoftau new bn128 22 pot22_0000.ptau -v && \
    snarkjs powersoftau contribute pot22_0000.ptau pot22_0001.ptau --name="First contribution" -v -e="random text" && \
    snarkjs powersoftau prepare phase2 pot22_0001.ptau pot22_final.ptau -v -e="nojan" && \
    snarkjs groth16 setup example.r1cs pot22_final.ptau example_0000.zkey && \
    snarkjs zkey contribute example_0000.zkey example_0001.zkey --name="1st Contributor Name" -v -e="random text" && \
    snarkjs zkey export verificationkey example_0001.zkey verification_key.json

# RUN snarkjs groth16 prove example_0001.zkey witness.wtns proof.json public.json

# RUN snarkjs groth16 verify verification_key.json public.json proof.json
