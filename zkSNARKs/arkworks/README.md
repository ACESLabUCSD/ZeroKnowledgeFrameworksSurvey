# Arkworks

## About the framework
This Rust [framework](https://github.com/arkworks-rs) provides a full ecosystem for zkSNARK programming. Their repository includes low-level zkSNARK components, such as finite field arithmetic and elliptic curve modules. Alongside this, they provide circuit building modules and several proving systems. This is a great ecosystem for beginner and experienced ZK developers.


## What's included
- Docker environment with Arkworks installed
- Benchmarks for Matrix Multiplication, SHA256 and Poseidon hash functions, and Merkle Tree verification

## Building environment and benchmarks

1. With docker installed, create the base image and benchmarks by running:
```
docker build -t arkworks .
```

## Building your own custom circuit and compiling

1. Copy one of our provided folders and modify `src/main.rs` accordingly
2. Add the following to the Dockerfile:
```
COPY <program>/ <program>/
WORKDIR <program>
RUN cargo run --release
WORKDIR ..
```
