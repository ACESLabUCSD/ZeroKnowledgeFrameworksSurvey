# Hyrax

## About the framework
This Python/C++ [framework](https://github.com/hyraxZK/hyraxZK?tab=readme-ov-file) implements the [Hyrax](https://ieeexplore.ieee.org/stamp/stamp.jsp?tp=&arnumber=8418646) protocol. The framework acts as a high-level repository for lower-level components. While this is presented as a very efficient backend for zkSNARKs without trusted setup, there is no high-level API exposed for building custom circuits. The examples prove how great this framework is, however the syntax is not something that can easily be deciphered by a new ZK developer.


## What's included
- Docker environment with Hyrax installed
- Benchmarks for Matrix Multiplication and SHA256

## Building environment and benchmarks

1. With docker installed, create the base image and benchmarks by running:
```
docker build -t hyrax .
```
