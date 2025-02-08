# Moz $ Z^2_K $ arella

## About the framework
This Rust [framework](https://github.com/AarhusCrypto/Mozzarella) implements the [Mozzarella](https://ia.cr/2022/819) protocol.


## What's included
- Docker environment with Mozzarella installed
- Benchmark for Matrix Multiplication, with two-party setting already set up for user
    - Matmul code can be found [here](https://github.com/nojansheybani/Mozzarella/blob/mozzarella/ocelot/src/bin/matrix_mul.rs), as we had to make modifications to get ring size working.
    - We were not able to get SHA256 working in this framework

## Building environment and benchmarks

1. With docker installed, create the base image and benchmarks by running:
```
docker build -t mozz .;  docker-compose up
```

## Building your own custom circuit and compiling

1. Build an `.rs` file using syntax similar to this [here](https://github.com/nojansheybani/Mozzarella/blob/mozzarella/ocelot/src/bin/matrix_mul.rs) - there are also examples in the `ocelot/quarksilver` directory of this repo
2. Add `COPY <file>.rs Mozzerella/ocelot/bin/`
3. Change program call in `docker-compose.yml`