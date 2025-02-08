// Adapted from https://github.com/amyzx/arkworks_learn/blob/main/src/main.rs and https://github.com/Pratyush/algebra-intro

use ark_r1cs_std::{fields::fp::FpVar, alloc::AllocVar};
use ark_r1cs_std::eq::EqGadget;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_bn254::{Bn254, Fr};
use ark_ff::{Zero};
use ark_groth16::*; // import Groth16 library
use ark_crypto_primitives::snark::*;
use ark_std::rand::{rngs::StdRng, SeedableRng};
use ark_serialize::*;
use std::time;

pub type Curve = Bn254;
pub type F = Fr;

const BENCHMARK_ROUNDS: u32 = 10;
const MATRIX_SIZE: usize = 8;

#[derive(Clone)]
struct MatmulCircuit {
    x: Vec<Vec<F>>,   // public input
    w: Vec<Vec<F>>,   // witness
    y: Vec<Vec<F>>,   // public input
}

// Generate R1CS for MatmulCircuit
impl ConstraintSynthesizer<F> for MatmulCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {

        // All matrices are MATRIX_SIZExMATRIX_SIZE
        let m = self.x.len();
        let t = self.x[0].len();
        let n = self.y[0].len();

        // Add X as public input
        for i in 0..m {
            for j in 0..t {
                let _x_input = FpVar::<F>::new_input(
                    ark_relations::ns!(cs, "new input gadget"), || Ok(self.x[i][j])
                ).expect("create new input");
            }
        }

        // Add W as witness
        for i in 0..t {
            for j in 0..n {
                let _w_witness = FpVar::<F>::new_witness(
                    ark_relations::ns!(cs, "new witness gadget"), || Ok(self.w[i][j])
                ).expect("create new witness");
            }
        }

        // matrix multiplication
        for j in 0..n {
            for i in 0..m {
                // Instantiate tmp_sum = 0
                let mut tmp_sum = FpVar::<F>::new_witness(
                    ark_relations::ns!(cs, "zero gadget"), || Ok(F::zero())
                ).expect("create zero ");

                // MAC Operations
                for k in 0..t {
                    tmp_sum += self.x[i][k] * self.w[k][j];
                }

                // Add Y as public input
                let y_input = FpVar::<F>::new_input(
                    ark_relations::ns!(cs, "new input gadget"), || Ok(self.y[i][j])
                ).expect("create new input");

                // Check X * W = Y
                y_input.enforce_equal(&tmp_sum).expect("enforce equal: x * w = y");
            }
        }
        Ok(())
    }
}

fn main() {
    let mut rng = StdRng::seed_from_u64(0u64);

    let mut setup_time = time::Duration::ZERO;
    let mut proof_time = time::Duration::ZERO;
    let mut proof_size = 0;
    let mut verify_time = time::Duration::ZERO;

    for _ in 0..BENCHMARK_ROUNDS {
        // Make zero value in field
        let val = i64::unsigned_abs(0);
        let zero_f: F = val.into();

        // Make MATRIX_SIZExMATRIX_SIZE 0 matrices
        let x = vec![vec![zero_f; MATRIX_SIZE]; MATRIX_SIZE];
        let w = vec![vec![zero_f; MATRIX_SIZE]; MATRIX_SIZE];
        let y = vec![vec![zero_f; MATRIX_SIZE]; MATRIX_SIZE];

        let circuit = MatmulCircuit {
            x : x.clone(),
            w : w.clone(),
            y : y.clone(),
        };

        // Read Public Inputs
        let mut statement = Vec::new();
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                statement.push(x[i][j].clone());
            }
        }
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                statement.push(y[i][j].clone());
            }
        }

        let mut t: time::Instant;

        t = time::Instant::now();
        let (pk, vk) = Groth16::<Curve>::setup(circuit.clone(), &mut rng).unwrap();
        let pvk = Groth16::<Curve>::process_vk(&vk).unwrap();
        setup_time += t.elapsed();

        t = time::Instant::now();
        let proof = Groth16::<Curve>::prove(&pk, circuit.clone(), &mut rng).unwrap();
        proof_time += t.elapsed();

        proof_size = proof.compressed_size();

        t = time::Instant::now();
        assert!(Groth16::<Curve>::verify_with_processed_vk(&pvk, &statement, &proof).unwrap());
        verify_time += t.elapsed();
    }

    println!("Setup time: {} us", (setup_time / BENCHMARK_ROUNDS).as_micros());
    println!("Proof time: {} us", (proof_time / BENCHMARK_ROUNDS).as_micros());
    println!("Proof size: {} B", proof_size);
    println!("Verify time: {} us", (verify_time / BENCHMARK_ROUNDS).as_micros());
}
