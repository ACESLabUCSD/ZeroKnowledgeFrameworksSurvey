# Halo2

## About the framework
This Rust [framework](https://github.com/zcash/halo2) implements the [Halo2](https://zcash.github.io/halo2/index.html) protocol. This framework was developed by Zcash and has some pretty good resources on gadgets that have been built and tutorials on building on gadgets [here](https://github.com/adria0/awesome-halo2). This framework is actively being developed.


## What's included
- Docker environment with Halo2 installed
- Docker environment with EZKL a Halo2-based ZKML framework
- Benchmark for Matrix Multiplication and SHA256

## Building environment and benchmarks

1. With docker installed, create the base image and benchmarks by running:
```
docker build . -t halo
```

NOTE: A recent update to Halo2 breaks our Matrix Multiplication - we will address this

## Building your own custom circuit and compiling

1. Build an `.rs` file using syntax similar to our examples or the referenced documentation
3. Add `COPY <file>.rs benches/<file>.rs` to the Dockerfile
4. Add `RUN cargo bench <file>`