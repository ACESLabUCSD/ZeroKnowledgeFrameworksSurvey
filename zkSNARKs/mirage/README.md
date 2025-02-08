# Mirage

## About the framework
This Java [framework](https://github.com/akosba/mirage) implements the [Mirage](https://www.usenix.org/system/files/sec20-kosba.pdf) protocol. Mirage provides a universal circuit generator built on top of a libsnark backend, to enable trusted setup for certain circuit sizes, rather than per circuit. The syntax that Mirage uses is readable, however there are certain parameters that are not explained well that require a deeper knowledge of the protocol, thus making this a bit unsuitable for beginner developers.


## What's included
- Docker environment with Mozzarella installed
- Benchmark for Matrix Multiplication and SHA256

NOTE: An recent update to one of Mirage's dependencies breaks our benchmarks - we are working on addressing this.

## Building environment and benchmarks

1. With docker installed, create the base image and benchmarks by running:
```
docker build -t mirage .
```

## Building your own custom circuit and compiling

1. Build an `.java` file using syntax similar to `SHA256Demo.java`
2. Add the following to the Dockerfile:
```
COPY <file>.java ./MirageCircuitGenerator/src/examples/
```
3. Add the following to the Dockerfile:
```
RUN cd MirageCircuitGenerator && java -cp bin examples.<file> && cp <file>.in univ_circuit.arith /mir/mirage/libsnark/build/libsnark/mirage_interface
```
4. Add the following to the Dockerfile:
```
RUN ./run_universal_gg_ppzksnark univ_circuit.arith <file>.in
```
