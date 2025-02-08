use ark_r1cs_std::{fields::fp::FpVar, alloc::AllocVar, fields::FieldVar};
use ark_r1cs_std::eq::EqGadget;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_bls12_381::{Bls12_381, Fr as BlsFr};
use ark_marlin::{Marlin};
use ark_std::rand::{rngs::StdRng, SeedableRng};
use std::time;
use ark_poly::univariate::DensePolynomial;
use ark_poly_commit::marlin_pc::MarlinKZG10;
use blake2::Blake2s;
use ark_serialize::*;

pub type Curve = Bls12_381;
pub type F = BlsFr;

const BENCHMARK_ROUNDS: u32 = 1;
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

        // All matrices are MATRIX_SIZE x MATRIX_SIZE
        let m = self.x.len();
        let t = self.x[0].len();
        let n = self.y[0].len();

        // Add X as public input
        let mut x_vars = Vec::new();
        for i in 0..m {
            let mut row_vars = Vec::new();
            for j in 0..t {
                let x_input = FpVar::<F>::new_input(
                    ark_relations::ns!(cs, "x_input"), || Ok(self.x[i][j])
                )?;
                row_vars.push(x_input);
            }
            x_vars.push(row_vars);
        }

        // Add W as witness
        let mut w_vars = Vec::new();
        for i in 0..t {
            let mut row_vars = Vec::new();
            for j in 0..n {
                let w_witness = FpVar::<F>::new_witness(
                    ark_relations::ns!(cs, "w_witness"), || Ok(self.w[i][j])
                )?;
                row_vars.push(w_witness);
            }
            w_vars.push(row_vars);
        }

        // Add Y as public input
        let mut y_vars = Vec::new();
        for i in 0..m {
            let mut row_vars = Vec::new();
            for j in 0..n {
                let y_input = FpVar::<F>::new_input(
                    ark_relations::ns!(cs, "y_input"), || Ok(self.y[i][j])
                )?;
                row_vars.push(y_input);
            }
            y_vars.push(row_vars);
        }

        // matrix multiplication
        for i in 0..m {
            for j in 0..n {
                let mut tmp_sum = FpVar::<F>::zero();
                for k in 0..t {
                    tmp_sum += &x_vars[i][k] * &w_vars[k][j];
                }
                y_vars[i][j].enforce_equal(&tmp_sum)?;
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

        // Make MATRIX_SIZE x MATRIX_SIZE 0 matrices
        let x = vec![vec![zero_f; MATRIX_SIZE]; MATRIX_SIZE];
        let w = vec![vec![zero_f; MATRIX_SIZE]; MATRIX_SIZE];
        let y = vec![vec![zero_f; MATRIX_SIZE]; MATRIX_SIZE];

        let circuit = MatmulCircuit {
            x : x.clone(),
            w : w.clone(),
            y : y.clone(),
        };

        // Read Public Inputs
        let mut public_inputs = Vec::new();
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                public_inputs.push(x[i][j]);
            }
        }
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                public_inputs.push(y[i][j]);
            }
        }

        let mut t: time::Instant;

        t = time::Instant::now();
        let num_constraints = MATRIX_SIZE * MATRIX_SIZE * MATRIX_SIZE + MATRIX_SIZE * MATRIX_SIZE * MATRIX_SIZE;
        let num_variables = 6 * MATRIX_SIZE * MATRIX_SIZE;

        type MarlinSetup = Marlin<BlsFr, MarlinKZG10<Bls12_381, DensePolynomial<BlsFr>>, Blake2s>;
        let srs = MarlinSetup::universal_setup(
            num_constraints,
            num_variables,
            num_variables * 3, // A sufficiently large size for the evaluation domain
            &mut rng
        ).unwrap();
        let (pk, vk) = MarlinSetup::index(&srs, circuit.clone()).unwrap();
        setup_time += t.elapsed();

        t = time::Instant::now();
        let proof = MarlinSetup::prove(&pk, circuit.clone(), &mut rng).unwrap();
        proof_time += t.elapsed();
        println!("Proof size: {} B", proof.serialized_size());
        proof.print_size_info();

        t = time::Instant::now();
        assert!(MarlinSetup::verify(&vk, &public_inputs, &proof, &mut rng).unwrap());
        verify_time += t.elapsed();
    }

    println!("Setup time: {} us", (setup_time / BENCHMARK_ROUNDS).as_micros());
    println!("Proof time: {} us", (proof_time / BENCHMARK_ROUNDS).as_micros());
    // println!("Proof size: {} B", proof.print_size_info());
    println!("Verify time: {} us", (verify_time / BENCHMARK_ROUNDS).as_micros());
}