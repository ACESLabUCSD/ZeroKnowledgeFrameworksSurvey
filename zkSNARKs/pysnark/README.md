# PySNARK

## About the framework
This Python [framework](https://github.com/meilof/pysnark) allows developers to build zk-SNARKs with pure Python. It also supports many integrations, including libsnark, snarkjs, bulletproofs, and qaptools. One of the most interesting use cases of PySNARK is its ability to produce Solidity smart contracts. Due to this being a Python framework, its runtime is inherently slower when compared to Rust/C++ programs. However, its ease of use is almost unparalleled and would be a great starting point for developers that are not worried about performance yet.

## What's included
- Docker environment with PySNARK installed
- Benchmarks for Matrix Multiplication and SHA256

## Building environment and benchmarks

1. With docker installed, create the base image and benchmarks by running:
```
docker build -t pysnark .
```

## Building your own custom circuit and compiling

1. Build a `.py` file using the same syntax as seen in our examples
2. Add the following to the Dockerfile:
```
COPY <file>.py <file>.py
RUN rm -f pysnark_*
RUN python3 <file>.py
```