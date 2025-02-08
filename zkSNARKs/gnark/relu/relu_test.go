package main

import (
	"fmt"
	"testing"
	"time"

	// "github.com/consensys/gnark-crypto/ecc"
	// "github.com/consensys/gnark/backend/groth16"

	// "github.com/consensys/gnark/frontend/cs/r1cs"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/groth16"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/r1cs"
	// "github.com/consensys/gnark/test"
)

func TestReluCircuit(t *testing.T) {
	// var api frontend.API
	// assert := test.NewAssert(t)
	// var circuit MatrixCircuit

	// assert.ProverSucceeded(&circuit, &MatrixCircuit{Mat1Val: 3, Mat2Val: 2, Mat3Val: 12})

	// fmt.Println("Hello, Module gnark!!")
	// // var circuit MatrixCircuit
	// Doubt #1 Witness Assignment is done in LINE #29 for int and LINE #34 for frontend.Variable
	ccs, err := frontend.Compile(ecc.BN254, r1cs.NewBuilder, &ReluCircuit{ReluVal: 4}) // These witness values are just for initializing, not really used for proving
	_ = err
	// groth16 zkSNARK: Setup
	pk, vk, err := groth16.Setup(ccs)
	// witness definition
	assignment := ReluCircuit{ReluVal: 4}
	witness, err := frontend.NewWitness(&assignment, ecc.BN254)
	publicWitness, _ := witness.Public()
	// groth16: Prove & Verify
	start := time.Now()

	proof, err := groth16.Prove(ccs, pk, witness)

	elapsed := time.Since(start)

	err2 := groth16.Verify(proof, vk, publicWitness)
	_ = err2
	fmt.Println("Time Elapsed: ", elapsed)
}
