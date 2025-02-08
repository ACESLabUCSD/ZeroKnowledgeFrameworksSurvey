# Gnark

## About the framework
This Go [framework](https://github.com/Consensys/gnark) provides a high-level API for designing ZK circuits with efficienct PLONK and SNARK backends. Gnark allows for development of complex circuits with very readable code. Alongside this, they provide excellent [documentation](https://docs.gnark.consensys.io/). Also, gnark is one of the only open-source frameworks that has been [partially audited for security](https://github.com/ConsenSys/gnark-crypto/blob/master/audit_oct2022.pdf), meaning that it is on the way to be production-ready.

## What's included
- Docker environment with Gnark installed
- Benchmarks for Matrix Multiplication, SHA256, MiMC, and ReLU, using a zk-SNARK Groth16 Prover, PLONK with Kate commitments prover, and PLONK with FRI prover

NOTE: A recent update in Gnark has broken our PLONK benchmarks, but we will be addressing this and pushing an updated version soon!

## Building environment and benchmarks

1. With docker installed, create the base image and benchmarks by running:
```
docker build . -t gnark
```

## Building your own custom circuit and compiling

1. Copy one of our provided folders and modify the `.go` files accordingly
2. Add the following to the Dockerfile:
```
COPY <program>/ <program>/
WORKDIR <program>
RUN go test -v
RUN go run . groth16
RUN go run . plonk-kzg
RUN go run . plonk-fri
WORKDIR ..
```
