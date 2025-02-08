package main

import (
	"bytes"
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

type MatrixCircuit struct {
	Mat1Val frontend.Variable
	Mat2Val frontend.Variable
	Mat3Val frontend.Variable
}

func (circuit *MatrixCircuit) Define(api frontend.API) error {
	n := 32

	var Mat1 [512 * 512]frontend.Variable
	var Mat2 [512 * 512]frontend.Variable
	var Mat3 [512 * 512]frontend.Variable

	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			Mat1[i*n+j] = circuit.Mat1Val
			Mat2[i*n+j] = circuit.Mat2Val
			Mat3[i*n+j] = circuit.Mat3Val
			// fmt.Println(i, j)
		}
	}

	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			x3 := api.Add(0, 0)
			for k := 0; k < n; k++ {
				x3 = api.Add(x3, api.Mul(Mat1[i*n+k], Mat2[k*n+j]))
				// fmt.Println("DEFINING")
			}
			api.AssertIsEqual(Mat3[i*n+j], x3)
		}
	}

	// x3 := api.Mul(circuit.Mat1Val, circuit.Mat2Val)
	// api.AssertIsEqual(circuit.Mat3Val, x3)
	fmt.Println("DEFINING")
	return nil
}

func benchmarkMatMulWithGroth16() {
	var setupTime time.Duration
	var proofTime time.Duration
	var proofSize = 0
	var verifyTime time.Duration

	for n := 0; n < BENCHMARK_ROUNDS; n++ {
		r1cs, _ := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, &MatrixCircuit{Mat1Val: 3, Mat2Val: 2, Mat3Val: 12}) // These witness values are just for initializing, not really used for proving

		var t = time.Time{}

		// perform Groth16 setup
		t = time.Now()
		pk, vk, _ := groth16.Setup(r1cs)
		setupTime += time.Since(t)

		// witness definition
		assignment := MatrixCircuit{Mat1Val: 3, Mat2Val: 2, Mat3Val: 192}
		witness, _ := frontend.NewWitness(&assignment, ecc.BN254.ScalarField())
		publicWitness, _ := witness.Public()

		// generate a Groth16 proof
		t = time.Now()
		proof, _ := groth16.Prove(r1cs, pk, witness)
		proofTime += time.Since(t)

		var buf bytes.Buffer
		// proof.WriteRawTo(&buf) // uncompressed
		proof.WriteTo(&buf) // compressed
		proofSize = buf.Len()

		// verify the Groth16 proof
		t = time.Now()
		_ = groth16.Verify(proof, vk, publicWitness)
		verifyTime += time.Since(t)
	}

	fmt.Printf("Setup time: %v us\n", (setupTime / BENCHMARK_ROUNDS).Microseconds())
	fmt.Printf("Proof time: %v us\n", (proofTime / BENCHMARK_ROUNDS).Microseconds())
	fmt.Printf("Proof size: %v B\n", proofSize)
	fmt.Printf("Verify time: %v us\n", (verifyTime / BENCHMARK_ROUNDS).Microseconds())
}

func benchmarkMatMulWithPlonkKzg() {
	var setupTime time.Duration
	var proofTime time.Duration
	var proofSize = 0
	var verifyTime time.Duration

	for n := 0; n < BENCHMARK_ROUNDS; n++ {
		scs, _ := frontend.Compile(ecc.BN254.ScalarField(), scs.NewBuilder, &MatrixCircuit{Mat1Val: 3, Mat2Val: 2, Mat3Val: 12}) // These witness values are just for initializing, not really used for proving

		var t = time.Time{}

		// perform PlonK-KZG setup
		_r1cs := scs.(*csBn254.SparseR1CS)
		srs, _ := test.NewKZGSRS(_r1cs) // KZG SRS is supposed to be created in advance.
		t = time.Now()
		pk, vk, _ := plonk.Setup(_r1cs, srs)
		setupTime += time.Since(t)

		// witness definition
		assignment := MatrixCircuit{Mat1Val: 3, Mat2Val: 2, Mat3Val: 192}
		witness, _ := frontend.NewWitness(&assignment, ecc.BN254.ScalarField())
		publicWitness, _ := witness.Public()

		// generate a PlonK-KZG proof
		t = time.Now()
		proof, _ := plonk.Prove(scs, pk, witness)
		proofTime += time.Since(t)

		var buf bytes.Buffer
		// proof.WriteRawTo(&buf) // uncompressed
		proof.WriteTo(&buf) // compressed
		proofSize = buf.Len()

		// verify the PlonK-KZG proof
		t = time.Now()
		_ = plonk.Verify(proof, vk, publicWitness)
		verifyTime += time.Since(t)
	}

	fmt.Printf("Setup time: %v us\n", (setupTime / BENCHMARK_ROUNDS).Microseconds())
	fmt.Printf("Proof time: %v us\n", (proofTime / BENCHMARK_ROUNDS).Microseconds())
	fmt.Printf("Proof size: %v B\n", proofSize)
	fmt.Printf("Verify time: %v us\n", (verifyTime / BENCHMARK_ROUNDS).Microseconds())
}

func benchmarkMatMulWithPlonkFri() {
	var setupTime time.Duration
	var proofTime time.Duration
	var proofSize = 0
	var verifyTime time.Duration

	for n := 0; n < BENCHMARK_ROUNDS; n++ {
		scs, _ := frontend.Compile(ecc.BN254.ScalarField(), scs.NewBuilder, &MatrixCircuit{Mat1Val: 3, Mat2Val: 2, Mat3Val: 12}) // These witness values are just for initializing, not really used for proving

		var t = time.Time{}

		// perform PlonK-FRI setup
		// _r1cs := scs.(*csBn254.SparseR1CS)
		t = time.Now()
		pk, vk, _ := plonkfri.Setup(scs)
		setupTime += time.Since(t)

		// witness definition
		assignment := MatrixCircuit{Mat1Val: 3, Mat2Val: 2, Mat3Val: 192}
		witness, _ := frontend.NewWitness(&assignment, ecc.BN254.ScalarField())
		publicWitness, _ := witness.Public()

		// generate a PlonK-FRI proof
		t = time.Now()
		proof, _ := plonkfri.Prove(scs, pk, witness)
		proofTime += time.Since(t)

		// var buf bytes.Buffer
		// proof.WriteRawTo(&buf) // uncompressed
		// proof.WriteTo(&buf) // compressed
		// proofSize = buf.Len()
		proofSize = -1

		// verify the PlonK-FRI proof
		t = time.Now()
		_ = plonkfri.Verify(proof, vk, publicWitness)
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
		benchmarkMatMulWithGroth16()
	case "plonk-kzg":
		benchmarkMatMulWithPlonkKzg()
	case "plonk-fri":
		benchmarkMatMulWithPlonkFri()
	default:
		fmt.Printf("Invalid choice: %s\n", proofSystem)
		printUsage()
		os.Exit(1)
	}
}
