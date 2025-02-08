## dusk-network's PLONK environment

Using plonk [environment](https://github.com/dusk-network/plonk)

# Dusk PLONK

## About the framework
This Rust [crate](https://github.com/dusk-network/plonk?tab=readme-ov-file) provides a pure Rust implementation of a PLONK prover over the BLS12-381 elliptic curve.


## What's included
- Docker environment with dusk_plonk installed
- Baseline benchmark of the underlying PLONK scheme

## Building environment and benchmarks

1. With docker installed, create the base image and benchmarks by running:
```
docker build . -t plonk
```