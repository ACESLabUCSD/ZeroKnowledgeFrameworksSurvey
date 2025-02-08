# RISC Zero

## About the framework
This Rust [zkVM](https://github.com/risc0/risc0) is based on the RISC-V architecture and uses the same instruction set. RISC Zero allows for verifiable computing of programs using RISC-V instructions. This zkVM supports so many different integrations, and works with all existing Rust crates, which makes it such a powerful framework. Although zkVMs are slow for one-off programs, this is a great place for ZK developers to start with STARK development if they can adopt the RISC Zero ecosystem in production. The developers claim that they support GPU acceleration of the proof generation, however we ran into many errors when trying this.

## What's included
- Docker environment with Mozzarella installed
- Benchmarks for Matrix Multiplication and SHA256

## Building environment and benchmarks

1. With docker installed, create the base image and benchmarks by running:
```
docker build -t risczero .
```

## Building your own custom circuit and compiling

We recommend to the excellent introduction written by the RISC Zero team [here](https://dev.risczero.com/api/zkvm/tutorials/hello-world)
