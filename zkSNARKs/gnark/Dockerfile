FROM golang:1.20

RUN mkdir /workspace

WORKDIR /workspace

RUN git clone https://github.com/ConsenSys/gnark.git

COPY gnark_extra/sha256 gnark/std/hash/sha256

WORKDIR gnark
COPY matmul matmul
COPY hash hash
RUN go mod tidy

WORKDIR matmul
RUN go test -v
RUN go run . groth16
RUN go run . plonk-kzg
RUN go run . plonk-fri
WORKDIR ..

WORKDIR hash
RUN go test -v
RUN go run . groth16
RUN go run . plonk-kzg
RUN go run . plonk-fri
WORKDIR ..
