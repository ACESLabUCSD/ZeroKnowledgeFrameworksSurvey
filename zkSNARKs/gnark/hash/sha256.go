// Based on https://github.com/ConsenSys/gnark/blob/79ecf530935e0202c096ce377c97acdcc2e23487/std/hash/sha256/sha256_test.go

package main

import (
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/std/hash/sha256"
)

type sha256Circuit struct {
	ExpectedResult [32]frontend.Variable `gnark:"y,public"`
	PreImage       []frontend.Variable   `gnark:"x"`
}

func (circuit *sha256Circuit) Define(api frontend.API) error {
	hashFunc := sha256.NewSHA256(api)
	result := hashFunc.Hash(circuit.PreImage)
	for i := 0; i < 32; i++ {
		api.AssertIsEqual(result[i], circuit.ExpectedResult[i])
	}
	return nil
}
