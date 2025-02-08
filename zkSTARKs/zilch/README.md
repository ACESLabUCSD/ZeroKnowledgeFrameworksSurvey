# Zilch

## About the framework
This Java [framework](https://github.com/TrustworthyComputing/Zilch) implements [Zilch](https://github.com/TrustworthyComputing/Zilch). As part of this, they propose an extension to the MIPS instruction set, called zMIPS. The developers provide a [front-end](https://github.com/TrustworthyComputing/ZeroJava-compiler). This is a very well-developed framework, but we ran into several compilation problems and memory overflow problems when compiling it on a powerful server. We are continuing development with this framework to see what can be done.


## What's included
- Docker environment with Zilch installed

NOTE: We are still developing benchmarks for this framework, as we found several internal problems during compilation of our previously developed benchmarks (which also caused memory overflow).

## Building environment and benchmarks

1. With docker installed, create the base image and benchmarks by running:
```
docker build -t zilch .
```