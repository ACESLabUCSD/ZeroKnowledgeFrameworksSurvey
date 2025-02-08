# SnarkJS

## About the framework
This JavaScript [framework](https://github.com/iden3/snarkjs) utilizes a [circom](https://github.com/iden3/circom) frontend which enables an extremely approachable way for developers to build ZK circuits. Due to the nature of JavaScript, snarkJS allows for proofs to be easily integrated into webpages with node.js. One thing to note about this library is that it relies on a powers of tau ceremony for trusted setup, which needs files that are too large for GitHub. We provide the script for generating these files, but it takes a while. Once you have the keys for phase 1 of trusted setup, you no longer need to re-run that part fo the Dockerfile.


## What's included
- Docker environment with snarkJS installed
- Benchmark for Matrix Multiplication and SHA256 (found in `../../zkCircuits/circom/`)
- Script for copying files from Docker container to local computer

## Building environment and benchmarks

1. With docker installed, create the base image and benchmarks by running:
```
docker build -t snarkjs .
```

## Building your own custom circuit and compiling

1. Build an `example.circom` file using `../../zkCircuits/circom/` and copy it to `src/`. Update `input.json` accordingly