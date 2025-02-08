# RapidSNARK

## About the framework
This JavaScript [framework](https://github.com/iden3/rapidsnark) is just snarkJS with an accelerated prover. Everything else about it is essentially adopted from snarkJS, as it is a follow-up work.


## What's included
- Docker environment with rapidsnark installed
- Benchmark for Matrix Multiplication and SHA256 (found in `../../zkCircuits/circom/`)
- Script for copying files from Docker container to local computer

## Building environment and benchmarks

1. With docker installed, create the base image and benchmarks by running:
```
docker build -t rapidsnark .
```

## Building your own custom circuit and compiling

1. Build an `example.circom` file using `../../zkCircuits/circom/` and copy it to `src/`. Update `input.json` accordingly