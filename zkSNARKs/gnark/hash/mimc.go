// Based on https://github.com/ConsenSys/gnark/blob/master/std/hash/mimc/mimc_test.go

package main

import (
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/std/hash/mimc"
)

type mimcCircuit struct {
	ExpectedResult frontend.Variable     `gnark:"y,public"`
	Data           [10]frontend.Variable `gnark:"x"`
}

func (circuit *mimcCircuit) Define(api frontend.API) error {
	hashFunc, err := mimc.NewMiMC(api)
	if err != nil {
		return err
	}
	hashFunc.Write(circuit.Data[:]...)
	result := hashFunc.Sum()
	api.AssertIsEqual(result, circuit.ExpectedResult)
	return nil
}
