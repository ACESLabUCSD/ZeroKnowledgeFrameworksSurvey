# Libsnark

## About the framework
This C++ [framework](https://github.com/scipr-lab/libsnark) has one of the most mature codebases in the ZK landscape. Libsnark provides a lot of different parameters that can be tweaked based on the applications, such as elliptic curve or proving scheme. Most importantly, libsnark integrates cleanly with [xjsnark](https://github.com/akosba/xjsnark), a high-level Java framework for building libsnark circuits with a built-in optimizer. This makes libsnark a great candidate for beginner developers with Java experience, as it allows the low-level details to be abstracted out.


## What's included
- Docker environment with libsnark installed
- Benchmarks for Matrix Multiplication, SHA256, and ReLU
- Script for copying files from Docker container to local container
- Examples of xJsnark generated files in `./circuits/`

## Building environment and benchmarks

1. With docker installed, create the base image and benchmarks by running:
```
docker build . -t libsnark; bash rundocker.sh
```

2. Once the shell inside the docker containter prompts you, run the demo
```
./main
```

## Building your own custom circuit and compiling

1. Build a `.cpp` file implementing your function in `src/`
2. Modify `src/main.cpp` to import and call your function
3. Change program call in `docker-compose.yml`
