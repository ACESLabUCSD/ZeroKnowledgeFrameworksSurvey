// Based on https://github.com/ConsenSys/gnark/blob/master/std/hash/mimc/mimc_test.go

package main

import (
	"math/big"
	"testing"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark-crypto/hash"
	"github.com/consensys/gnark/test"
)

func TestMimcHash(t *testing.T) {
	assert := test.NewAssert(t)

	curve := ecc.BLS12_381
	hashType := hash.MIMC_BLS12_381

	var circuit, witness, wrongWitness mimcCircuit

	modulus := curve.ScalarField()
	var data [10]big.Int
	data[0].Sub(modulus, big.NewInt(1))
	for i := 1; i < 10; i++ {
		data[i].Add(&data[i-1], &data[i-1]).Mod(&data[i], modulus)
	}

	// running MiMC outside circuit, independently
	hashFunc := hashType.New()
	for i := 0; i < 10; i++ {
		hashFunc.Write(data[i].Bytes())
	}
	correctHash := hashFunc.Sum(nil)

	// assert correctness against correct witness
	for i := 0; i < 10; i++ {
		witness.Data[i] = data[i].String()
	}
	witness.ExpectedResult = correctHash
	assert.SolvingSucceeded(&circuit, &witness, test.WithCurves(curve))

	// assert failure against wrong witness
	for i := 0; i < 10; i++ {
		wrongWitness.Data[i] = data[i].Sub(&data[i], big.NewInt(1)).String()
	}
	wrongWitness.ExpectedResult = correctHash
	assert.SolvingFailed(&circuit, &wrongWitness, test.WithCurves(curve))
}
