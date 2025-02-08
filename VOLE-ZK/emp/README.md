# EMP-zk

## About the framework
This C++ [framework](https://github.com/emp-toolkit/emp-zk) implements the [Wolverine](https://eprint.iacr.org/2020/925), [Quicksilver](https://eprint.iacr.org/2021/076), and [Mystique](Mystique) protocols. The underlying cryptography for this implementation is subfield Vector Oblivious Linear Evaluation (sVOLE).

## What's included
- Implementations of SHA256, matrix multiplication, ReLU, and Merkle Tree ZKPs in `src` with an accompanying Docker environment
    - This will return timing and communication data for all benchmarks.
    - These benchmarks provide parameters that the users can modify 
- Docker environment for ZKML powered by `EMP-zk` in `rosetta-env`. This utilizes a custom fork of Rosetta.

## Building environment and benchmarks

1. With docker installed, create the base image by running:
```
docker build -t emp .
```

2. To run tests, rebuild image and start services:
```
docker build -f Dockerfile2 . -t emp:0.1
docker-compose up
```

3. This environment compiles one example at a time with our design, so to test individual circuits, you will have to change the `.cpp` in `CMakeLists.txt` (line 28). You can also modify `CMakeLists.txt` to include more executables and update `docker-compose.yml` accordingly.

## Building your own custom circuit and compiling

1. Build a `.cpp` file in `src/`
2. Change `.cpp` file in `CMakeLists.txt` (line 28)
3. Rebuild docker environment and benchmarks