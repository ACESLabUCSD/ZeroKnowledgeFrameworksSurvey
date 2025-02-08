use ark_bn254::{Bn254, Fr};
use ark_crypto_primitives::{
    crh::{
        sha256::{
            constraints::{Sha256Gadget, UnitVar},
            Sha256,
        },
        CRHScheme, CRHSchemeGadget,
    },
    snark::{CircuitSpecificSetupSNARK, SNARK},
};
use ark_ff::Field;
use ark_groth16::Groth16;
use ark_r1cs_std::{alloc::AllocVar, eq::EqGadget, fields::fp::FpVar, uint8::UInt8, ToBytesGadget};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_serialize::*;
use ark_std::rand::{rngs::StdRng, RngCore, SeedableRng};
use std::time;

pub type Curve = Bn254;
pub type F = Fr;

const HASH_INPUT_LEN: usize = 512; // bits
const BENCHMARK_ROUNDS: u32 = 10;

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
        assert!(Groth16::<Curve>::verify_with_processed_vk(&pvk, &circuit.digest, &proof).unwrap());
        verify_time += t.elapsed();
    }

    println!("Setup time: {} us", (setup_time / BENCHMARK_ROUNDS).as_micros());
    println!("Proof time: {} us", (proof_time / BENCHMARK_ROUNDS).as_micros());
    println!("Proof size: {} B", proof_size);
    println!("Verify time: {} us", (verify_time / BENCHMARK_ROUNDS).as_micros());
}
