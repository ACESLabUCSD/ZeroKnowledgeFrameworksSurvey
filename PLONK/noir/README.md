# Noir

## About the framework
[Noir](https://github.com/noir-lang/noir) is a DSL for SNARK/PLONK proving systems. This DSL has great [documentation](noir-lang.org) and a pretty active development [community](https://github.com/noir-lang/awesome-noir). This work provides great features and is still under active development.


## What's included
- Docker environment with Noir installed
- Benchmarks for Matrix Multiplication and SHA256
- Script for generating long inputs
- Script for copying results from Docker container to localhost

## Building environment and benchmarks

1. Build Docker image:
```
docker build -t noir .
```

2. Start Docker container:
```
docker run --name noir -it noir
```

3. Go to any circuit project and execute `run.py`. For example, to launch SHA256 circuit demo:
```
cd /noir/sha256
./run.py
```

## Building your own custom circuit and compiling

1. Copy one of our included examples and change the name to your desired program `<program>/`
2. Add `COPY <program>/ <program>` to the Dockerfile
3. Change `src/main.nr` (program) and `Prover.toml` (inputs) accordingly
4. Add `WORKDIR <program>/` to Dockerfile, followed by `RUN ./run.py`
