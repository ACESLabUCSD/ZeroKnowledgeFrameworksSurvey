# LegoSNARK

## About the framework
This C++ [framework](https://github.com/imdea-software/legosnark) implements the [LegoSNARK](https://eprint.iacr.org/2019/142.pdf) protocol. This is an interesting protocol as it uses commmit-and-prove zkSNARKs. It uses a libsnark backend, which allows for users to use libsnark gadgets easily, but libsnark is a bit difficult to build custom gadgets with (without using xJsnark, which is not supported by LegoSNARK natively). Regardless, LegoSNARK provides a really unique approach with great results presented in their paper, albeit without a dedicated high-level API.


## What's included
- Docker environment with Mozzarella installed
- Benchmarks for Matrix Multiplication (command line parameter N sets matrix size to 2^N) and SHA256

## Building environment and benchmarks

1. With docker installed, create the base image and benchmarks by running:
```
docker build . -t lego
```

## Building your own custom circuit and compiling

1. Build a `.cc` file using the same syntax as `sha256.cc`
2. Add the following to your Dockerfile:
```
COPY <file>.cc examples/<file>.cc
RUN printf "\n\n%s\n%s\n\n" "add_executable(<file> <file>.cc)" "target_link_libraries(<file> snark legobasic)" >> ./examples/CMakeLists.txt
```
3. Add the following to the Dockerfile:
```
RUN make -j8 && ./examples/<file>
```