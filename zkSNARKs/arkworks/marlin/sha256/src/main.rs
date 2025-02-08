// use ark_bls12_381::{Bls12_381, Fr as BlsFr};
use ark_bn254::{Bn254, Fr}; // Removed unused imports
use ark_crypto_primitives::{
    crh::{
        sha256::{
            constraints::{Sha256Gadget, UnitVar},
            Sha256,
        },
        CRHScheme, CRHSchemeGadget,
    },
};
use ark_ff::Field;
use ark_ff::fields::models::Fp256;
use ark_r1cs_std::{alloc::AllocVar, fields::fp::FpVar, uint8::UInt8, ToBytesGadget};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_std::{rand::{rngs::StdRng, RngCore, SeedableRng}, vec::Vec}; // Corrected Vec import
use std::time;
use std::borrow::Borrow; // Import Borrow trait

// Import Marlin related crates
use ark_marlin::{Marlin};
use ark_poly::univariate::DensePolynomial;
use ark_poly_commit::marlin_pc::MarlinKZG10;
use blake2::Blake2s;
use ark_serialize::CanonicalSerialize;

const HASH_INPUT_LEN: usize = 512; // bits
const BENCHMARK_ROUNDS: u32 = 10;

pub type Curve = Bls12_381;
pub type F = BlsFr;
// pub type Curve = Bn254;
// pub type F = Fr;

#[derive(Clone)]
struct Sha256Circuit {
    message: Vec<u8>, // witness, 512 bits input to hash
    digest: [F; 2],   // public input, 256 bits output from hash
}

impl ConstraintSynthesizer<F> for Sha256Circuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        let message_var = UInt8::new_witness_vec(cs.clone(), &self.message)?;

        let digest_var_0 = FpVar::new_input(cs.clone(), || Ok(&self.digest[0]))?;
        let digest_var_1 = FpVar::new_input(cs.clone(), || Ok(&self.digest[1]))?;

        // CRH parameters are "nothing"
        let param_var = UnitVar::default();

        let computed_digest = <Sha256Gadget<F> as CRHSchemeGadget<Sha256, F>>::evaluate(&param_var, &message_var)?;

        let digest_bytes_0 = digest_var_0.to_bytes()?;
        let digest_bytes_1 = digest_var_1.to_bytes()?;
        let computed_digest_bytes = computed_digest.to_bytes()?;

        computed_digest_bytes[0..16].enforce_equal(&digest_bytes_0[0..16])?;
        computed_digest_bytes[16..32].enforce_equal(&digest_bytes_1[0..16])?;

        Ok(())
    }
}

// fn message_var_from_bytes<F: Field>(
//     cs: ConstraintSystemRef<F>,
//     bytes: &[u8],
// ) -> Result<Vec<UInt8<F>>, SynthesisError> {
//     let mut message_var = Vec::new();
//     for byte in bytes {
//         message_var.push(UInt8::new_witness(cs.clone(), || Ok(*byte))?);
//     }
//     Ok(message_var)
// }

fn main() {
    let mut rng = StdRng::seed_from_u64(0u64);

    let mut setup_time = time::Duration::ZERO;
    let mut proof_time = time::Duration::ZERO;
    let mut proof_size = 0;
    let mut verify_time = time::Duration::ZERO;

    for _ in 0..BENCHMARK_ROUNDS {
        // Make a random byte-string of the given length
        let mut input = vec![0u8; HASH_INPUT_LEN / 8];
        rng.fill_bytes(&mut input);

        let expected_output = <Sha256 as CRHScheme>::evaluate(&(), input.clone()).unwrap();

        let circuit = Sha256Circuit {
            message: input.clone(),
            digest: [
                F::from_random_bytes(&expected_output[0..16]).unwrap(),
                F::from_random_bytes(&expected_output[16..32]).unwrap(),
            ],
        };

        // let dummy_cs = ConstraintSystemRef::None;
        // let dummy_circuit = circuit.clone();
        // dummy_circuit.generate_constraints(dummy_cs.clone()).unwrap();
        let num_constraints = 100000000;
        let num_variables = 100000000 + 100000000;

        let mut t: time::Instant;

        t = time::Instant::now();

        type MarlinSetup = Marlin<Fr, MarlinKZG10<Bls12_381, DensePolynomial<BlsFr>>, Blake2s>;
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

        t = time::Instant::now();
        assert!(MarlinSetup::verify(&vk, &circuit.digest, &proof, &mut rng).unwrap());
        verify_time += t.elapsed();
    }

    println!("Setup time: {} us", (setup_time / BENCHMARK_ROUNDS).as_micros());
    println!("Proof time: {} us", (proof_time / BENCHMARK_ROUNDS).as_micros());
    println!("Proof size: {} B", proof_size);
    println!("Verify time: {} us", (verify_time / BENCHMARK_ROUNDS).as_micros());
}