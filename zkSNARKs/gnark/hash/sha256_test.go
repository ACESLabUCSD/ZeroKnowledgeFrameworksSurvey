// Based on https://github.com/ConsenSys/gnark/blob/79ecf530935e0202c096ce377c97acdcc2e23487/std/hash/sha256/sha256_test.go

package main

import (
	"encoding/hex"
	"testing"

	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/test"
)

func TestSha256Hash(t *testing.T) {
	assert := test.NewAssert(t)

	input := "Hello, World!"
	outputHex := "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f"

	inputBytes := []byte(input)
	outputBytes, _ := hex.DecodeString(outputHex)

	// witness values preparation
	witness := sha256Circuit{
		PreImage:       make([]frontend.Variable, len(inputBytes)),
		ExpectedResult: [32]frontend.Variable{},
	}
	for i := 0; i < len(inputBytes); i++ {
		witness.PreImage[i] = int(inputBytes[i])
	}
	for i := 0; i < len(outputBytes); i++ {
		witness.ExpectedResult[i] = int(outputBytes[i])
	}

	// circuit preparation
	circuit := sha256Circuit{
		PreImage: make([]frontend.Variable, len(inputBytes)),
	}

	assert.SolvingSucceeded(&circuit, &witness)
}
