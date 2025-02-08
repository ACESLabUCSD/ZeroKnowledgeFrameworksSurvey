# Limbo

## About the framework
This C++ [framework](https://github.com/KULeuven-COSIC/Limbo) implements the [Limbo](https://acmccs.github.io/papers/p2087-amesA.pdf) protocol. Circuits for this protocol are represented as Bristol Fashion circuits (MPC-friendly circuits), which can be built in `../zkCircuits/emp-bristol`. More info about Bristol Circuits (BC) can be found [here](https://nigelsmart.github.io/MPC-Circuits/).

Note: Our BC generator does not automatically generate circuits in Limbo format. Users will need to add a line after line 2 that specifies the output (look at `matmul64.txt` for an example, or at aforementioned link). 

## What's included
- Docker environment with Limbo installed
- Bristol Circuit for matrix multiplication - we use a precomputed circuit from [here](https://nigelsmart.github.io/MPC-Circuits/) for SHA256 benchmark

## Building environment and benchmarks

1. With docker installed, create the base image and benchmarks by running:
```
docker build . -t limbo
```

## Building your own custom circuit and compiling

1. Build a `.txt` file using our BC generator in `../zkCircuits/emp-bristol`
2. Make necessary modifications (adding output)
3. Add test case for your new file in `Dockerfile` (just like lines 20 and 21)