package main

import (
	"bytes"
	"crypto/rand"
	"crypto/sha256"
	"fmt"
	"os"
	"time"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/groth16"
	"github.com/consensys/gnark/backend/plonk"
	"github.com/consensys/gnark/backend/plonkfri"
	csBn254 "github.com/consensys/gnark/constraint/bn254"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/r1cs"
	"github.com/consensys/gnark/frontend/cs/scs"
	"github.com/consensys/gnark/test"
)

const BENCHMARK_ROUNDS = 10
const HASH_INPUT_LEN = 512 // bits

func generateRandomBytes(n int) ([]byte, error) {
	b := make([]byte, n)
	_, err := rand.Read(b)
	if err != nil {
		return nil, err
	}
	return b, nil
}

func benchmarkSha256WithGroth16() {
	var setupTime time.Duration
	var proofTime time.Duration
	var proofSize = 0
	var verifyTime time.Duration

	for n := 0; n < BENCHMARK_ROUNDS; n++ {
		// Crashes when tested on 64 bytes, for unknown reason.
		inputBytes, err := generateRandomBytes(HASH_INPUT_LEN/8 + 1)
		if err != nil {
			panic(fmt.Errorf("failed to build random message: %v", err))
		}
		outputBytes := sha256.Sum256(inputBytes)

		// witness values preparation
		assignment := sha256Circuit{
			PreImage:       make([]frontend.Variable, len(inputBytes)),
			ExpectedResult: [32]frontend.Variable{},
		}
		for i := 0; i < len(inputBytes); i++ {
			assignment.PreImage[i] = int(inputBytes[i])
		}
		for i := 0; i < len(outputBytes); i++ {
			assignment.ExpectedResult[i] = int(outputBytes[i])
		}

		// circuit preparation
		circuit := sha256Circuit{
			PreImage: make([]frontend.Variable, len(inputBytes)),
		}

		r1cs, err := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, &circuit)
		if err != nil {
			panic(fmt.Errorf("failed to compile circuit: %v", err))
		}

		witness, err := frontend.NewWitness(&assignment, ecc.BN254.ScalarField())
		publicWitness, err := witness.Public()

		var t = time.Time{}

		// run Groth16 setup phase
		t = time.Now()
		pk, vk, err := groth16.Setup(r1cs)
		if err != nil {
			panic(fmt.Errorf("failed to run setup: %v", err))
		}
		setupTime += time.Since(t)

		// generate a Groth16 proof
		t = time.Now()
		proof, err := groth16.Prove(r1cs, pk, witness)
		if err != nil {
			panic(fmt.Errorf("failed to generate proof: %v", err))
		}
		proofTime += time.Since(t)

		var buf bytes.Buffer
		// proof.WriteRawTo(&buf) // uncompressed
		proof.WriteTo(&buf) // compressed
		proofSize = buf.Len()

		// verify the Groth16 proof
		t = time.Now()
		err = groth16.Verify(proof, vk, publicWitness)
		if err != nil {
			panic(fmt.Errorf("failed to verify proof: %v", err))
		}
		verifyTime += time.Since(t)
	}

	fmt.Printf("Setup time: %v us\n", (setupTime / BENCHMARK_ROUNDS).Microseconds())
	fmt.Printf("Proof time: %v us\n", (proofTime / BENCHMARK_ROUNDS).Microseconds())
	fmt.Printf("Proof size: %v B\n", proofSize)
	fmt.Printf("Verify time: %v us\n", (verifyTime / BENCHMARK_ROUNDS).Microseconds())
}

func benchmarkSha256WithPlonkKzg() {
	var setupTime time.Duration
	var proofTime time.Duration
	var proofSize = 0
	var verifyTime time.Duration

	for n := 0; n < BENCHMARK_ROUNDS; n++ {
		// Crashes when tested on 64 bytes, for unknown reason.
		inputBytes, err := generateRandomBytes(HASH_INPUT_LEN/8 + 1)
		if err != nil {
			panic(fmt.Errorf("failed to build random message: %v", err))
		}
		outputBytes := sha256.Sum256(inputBytes)

		// witness values preparation
		assignment := sha256Circuit{
			PreImage:       make([]frontend.Variable, len(inputBytes)),
			ExpectedResult: [32]frontend.Variable{},
		}
		for i := 0; i < len(inputBytes); i++ {
			assignment.PreImage[i] = int(inputBytes[i])
		}
		for i := 0; i < len(outputBytes); i++ {
			assignment.ExpectedResult[i] = int(outputBytes[i])
		}

		// circuit preparation
		circuit := sha256Circuit{
			PreImage: make([]frontend.Variable, len(inputBytes)),
		}

		scs, err := frontend.Compile(ecc.BN254.ScalarField(), scs.NewBuilder, &circuit)
		if err != nil {
			panic(fmt.Errorf("failed to compile circuit: %v", err))
		}

		witness, err := frontend.NewWitness(&assignment, ecc.BN254.ScalarField())
		publicWitness, err := witness.Public()

		var t = time.Time{}

		// run PlonK-KZG setup phase
		_r1cs := scs.(*csBn254.SparseR1CS)
		srs, err := test.NewKZGSRS(_r1cs) // KZG SRS is supposed to be created in advance.
		t = time.Now()
		pk, vk, err := plonk.Setup(scs, srs)
		if err != nil {
			panic(fmt.Errorf("failed to run setup: %v", err))
		}
		setupTime += time.Since(t)

		// generate a PlonK-KZG proof
		t = time.Now()
		proof, err := plonk.Prove(scs, pk, witness)
		if err != nil {
			panic(fmt.Errorf("failed to generate proof: %v", err))
		}
		proofTime += time.Since(t)

		var buf bytes.Buffer
		// proof.WriteRawTo(&buf) // uncompressed
		proof.WriteTo(&buf) // compressed
		proofSize = buf.Len()

		// verify the PlonK-KZG proof
		t = time.Now()
		err = plonk.Verify(proof, vk, publicWitness)
		if err != nil {
			panic(fmt.Errorf("failed to verify proof: %v", err))
		}
		verifyTime += time.Since(t)
	}

	fmt.Printf("Setup time: %v us\n", (setupTime / BENCHMARK_ROUNDS).Microseconds())
	fmt.Printf("Proof time: %v us\n", (proofTime / BENCHMARK_ROUNDS).Microseconds())
	fmt.Printf("Proof size: %v B\n", proofSize)
	fmt.Printf("Verify time: %v us\n", (verifyTime / BENCHMARK_ROUNDS).Microseconds())
}

func benchmarkSha256WithPlonkFri() {
	var setupTime time.Duration
	var proofTime time.Duration
	var proofSize = 0
	var verifyTime time.Duration

	for n := 0; n < BENCHMARK_ROUNDS; n++ {
		// Crashes when tested on 64 bytes, for unknown reason.
		inputBytes, err := generateRandomBytes(HASH_INPUT_LEN/8 + 1)
		if err != nil {
			panic(fmt.Errorf("failed to build random message: %v", err))
		}
		outputBytes := sha256.Sum256(inputBytes)

		// witness values preparation
		assignment := sha256Circuit{
			PreImage:       make([]frontend.Variable, len(inputBytes)),
			ExpectedResult: [32]frontend.Variable{},
		}
		for i := 0; i < len(inputBytes); i++ {
			assignment.PreImage[i] = int(inputBytes[i])
		}
		for i := 0; i < len(outputBytes); i++ {
			assignment.ExpectedResult[i] = int(outputBytes[i])
		}

		// circuit preparation
		circuit := sha256Circuit{
			PreImage: make([]frontend.Variable, len(inputBytes)),
		}

		scs, err := frontend.Compile(ecc.BN254.ScalarField(), scs.NewBuilder, &circuit)
		if err != nil {
			panic(fmt.Errorf("failed to compile circuit: %v", err))
		}

		witness, err := frontend.NewWitness(&assignment, ecc.BN254.ScalarField())
		publicWitness, err := witness.Public()

		var t = time.Time{}

		// run PlonK-FRI setup phase
		// _r1cs := scs.(*csBn254.SparseR1CS)
		t = time.Now()
		pk, vk, err := plonkfri.Setup(scs)
		if err != nil {
			panic(fmt.Errorf("failed to run setup: %v", err))
		}
		setupTime += time.Since(t)

		// generate a PlonK-FRI proof
		t = time.Now()
		proof, err := plonkfri.Prove(scs, pk, witness)
		if err != nil {
			panic(fmt.Errorf("failed to generate proof: %v", err))
		}
		proofTime += time.Since(t)

		// var buf bytes.Buffer
		// proof.WriteRawTo(&buf) // uncompressed
		// proof.WriteTo(&buf) // compressed
		// proofSize = buf.Len()
		proofSize = -1

		// verify the PlonK-FRI proof
		t = time.Now()
		err = plonkfri.Verify(proof, vk, publicWitness)
		if err != nil {
			panic(fmt.Errorf("failed to verify proof: %v", err))
		}
		verifyTime += time.Since(t)
	}

	fmt.Printf("Setup time: %v us\n", (setupTime / BENCHMARK_ROUNDS).Microseconds())
	fmt.Printf("Proof time: %v us\n", (proofTime / BENCHMARK_ROUNDS).Microseconds())
	fmt.Printf("Proof size: %v B\n", proofSize)
	fmt.Printf("Verify time: %v us\n", (verifyTime / BENCHMARK_ROUNDS).Microseconds())
}

func printUsage() {
	fmt.Printf("\nUsage: %s <groth16 OR plonk-kzg OR plonk-fri>\n", os.Args[0])
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Wrong usage.")
		printUsage()
		os.Exit(1)
	}

	proofSystem := os.Args[1]
	switch proofSystem {
	case "groth16":
		benchmarkSha256WithGroth16()
	case "plonk-kzg":
		benchmarkSha256WithPlonkKzg()
	case "plonk-fri":
		benchmarkSha256WithPlonkFri()
	default:
		fmt.Printf("Invalid choice: %s\n", proofSystem)
		printUsage()
		os.Exit(1)
	}
}
