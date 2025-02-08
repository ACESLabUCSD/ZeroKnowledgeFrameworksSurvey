package main

import (
	"testing"

	// "github.com/consensys/gnark-crypto/ecc"
	// "github.com/consensys/gnark/backend/groth16"
	"github.com/consensys/gnark/frontend"
	// "github.com/consensys/gnark/frontend/cs/r1cs"
)

type ReluCircuit struct {
	ReluVal frontend.Variable
}

func (circuit *ReluCircuit) Define(api frontend.API) error {
	n := 8
	// fmt.Println("Size of Relu array: ", n)

	var InpArr [4096]int
	var ReluArr [4096]frontend.Variable
	var zero frontend.Variable

	for i := 0; i < n; i++ {
		if i%2 == 0 {
			InpArr[i] = 5
		} else {
			InpArr[i] = 3
		}

	}

	threshold := circuit.ReluVal
	zero = 0

	for i := 0; i < n; i++ {
		if InpArr[i] <= 4 {
			ReluArr[i] = zero
		} else {
			ReluArr[i] = InpArr[i]
		}
	}

	for i := 0; i < n; i++ {
		// cmp = -1 if InpArr[i] > X0, 0 if InpArr[i] = X0, and 1 if InpArr[i] < X0
		// We have to make values boolean (0,1)
		// cond := 1 if above threshold, 0 if else
		cond := api.IsZero(api.Sub(api.Cmp(InpArr[i], threshold), 1))
		mux := api.Select(cond, InpArr[i], zero)
		api.AssertIsEqual(mux, ReluArr[i])
	}

	return nil
}

func main() {
	var t *testing.T
	TestReluCircuit(t)
}
