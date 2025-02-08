## Spartan

Built using [Spartan](https://eprint.iacr.org/2019/550.pdf)

To run:
```
docker build -t spartan .
```

# libspartan

## About the framework
This Rust [framework](https://github.com/microsoft/Spartan) implements the [Spartan](https://eprint.iacr.org/2019/550.pdf) protocol. This work is a fantastic implementation of a transparent zkSNARK with sub-linear verification costs. Amongst the literature, this is still viewed as a state-of-the-art protocol, and the repository is extensively thorough. The main drawback of this framework is that custom application development is done by manually describing the R1CS constraints, which is often very difficult, especially as the circuit scales up. Nevertheless, this framework is outstanding and, with the right frontend, can be very useful.

NOTE: We are currently working on a way to compile `zkinterface` files to a `libspartan` backend


## What's included
- Docker environment with libspartan installed

## Building environment and benchmarks

1. With docker installed, create the base image and benchmarks by running:
```
docker build -t libspartan .
```
