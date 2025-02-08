# libiop

## About the framework
This C++ [framework](https://github.com/scipr-lab/libiop?tab=readme-ov-file) implements the [Ligero](https://acmccs.github.io/papers/p2087-amesA.pdf), [Aurora](https://eprint.iacr.org/2018/828), and [Fractal](https://eprint.iacr.org/2019/1076) protocols. While these protocols are technically "IOP-based zkSNARKs", we found them a bit hard to categorize, as they Ligero carries traits that are very similar to MPC-in-the-head protocols, as their backend is primarily MPC. In our paper, we class Aurora and Fractal as zk-SNARKs, but we also believe it would be appropriate to classify them as zk-STARKs due to their underlying arithmetic and transparency.

NOTE: This work did not have a frontend that was intuitive to use (at least for us), but they did provide a way to test the performance of circuits based on the number of constraints, variables, and outputs. We provide a script for this in this folder, alongside tools to build R1CS representations of circuits (`../../zkCircuits/`) and a way to read these R1CS files (`../../r1csReader/`).


## What's included
- Docker environment with libiop installed
- Benchmarking script for Ligero
- Minor fixes that allow compilation of libiop (which was not possible before)

## Building environment and benchmarks

1. With docker installed, create the base image and benchmarks by running:
```
docker build . -t libiop
```

## Building your own custom circuit and compiling

1. Build an `.r1cs` file using `../../zkCircuits/Zokrates/`
2. Read the file to extract parameters with `../../r1csReader/` and update `test_matmul_snark.cpp` accordingly

NOTE: We only have included Ligero, but Aurora and Fractal will be added