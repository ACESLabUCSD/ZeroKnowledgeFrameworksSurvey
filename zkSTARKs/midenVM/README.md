## Miden Virtual Machine

Built using the Miden VM [environment](https://wiki.polygon.technology/docs/miden/intro/main)

To run:
```
docker build -t miden .
```

# Miden VM

## About the framework
This Rust [zkVM](https://github.com/AarhusCrypto/Mozzarella) generated a STARK proof of execution for every program that is run within it. They provide a great deal of [documentation](https://0xpolygonmiden.github.io/miden-vm/), but they admit that this work is in need of a front-end, as they only provide a standard library and low-level instruction set. Users also need a general idea of how a VM and, more importantly, a memory stack works. Luckily, the standard library is being extended weekly, while we wait for integration with a frontend that can compile to the Miden assembly. For one-off programs, this is a pretty slow framework, but that is only because it is a zkVM and not a stand-alone zkSTARK framework.


## What's included
- Docker environment with Miden VM installed
- Benchmark for Matrix Multiplication, SHA256, and Fibonacci sequence
    - This framework doesn't support matrix operations, currently, so we provide a low-level, makeshift matrix multiplication benchmark that performs all the additions and integer multiplications that would be present in a matrix multiplication, with the final memory stack being checked.

## Building environment and benchmarks

1. With docker installed, create the base image and benchmarks by running:
```
docker build -t miden .
```

## Building your own custom circuit and compiling

1. Copy one of our benchmark folders for your new program `<program>/`
2. Change `.inputs` and `.masm` accordingly
3. Add the following to your Dockerfile:
```
RUN time ./target/optimized/miden prove -a <program>/<program>.masm -n <outputs>
RUN time ./target/optimized/miden verify -p <program>/<program>.proof -h <program hash>
```