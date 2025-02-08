// Based on https://github.com/arkworks-rs/crypto-primitives/blob/main/src/crh/sha256/constraints.rs

use ark_crypto_primitives::crh::{
    CRHSchemeGadget, CRHScheme,
    sha256::{constraints::{UnitVar, Sha256Gadget}, Sha256}
};
use ark_bls12_377::Fr;
use ark_r1cs_std::bits::uint8::UInt8;
use ark_relations::{r1cs::{ConstraintSystem}, ns};
use ark_std::rand::RngCore;
use ark_r1cs_std::R1CSVar;

const INPUT_LEN: usize = 128;

pub fn test() {
    println!("start: setup");
    let mut rng = ark_std::test_rng();
    let cs = ConstraintSystem::<Fr>::new_ref();

    // CRH parameters are nothing
    let param = ();
    let param_var = UnitVar::default();

    // Make a random byte-string of the given length
    let mut input = vec![0u8; INPUT_LEN];
    rng.fill_bytes(&mut input);

    let input_var = UInt8::new_witness_vec(ns!(cs, "input"), &input).unwrap();
    println!("finish: setup");

    println!("start: hash");
    // Compute the hash
    let computed_output = <Sha256Gadget<Fr> as CRHSchemeGadget<Sha256, Fr>>::evaluate(
        &param_var,
        &input_var,
    ).unwrap();
    println!("finish: hash");

    // Compute the correct hash
    let expected_output = <Sha256 as CRHScheme>::evaluate(&param, input).unwrap();

    // Assert consistency
    assert_eq!(
        computed_output.value().unwrap().to_vec(),
        expected_output,
        "CRH mismatch"
    );
    println!("test passed");
}
