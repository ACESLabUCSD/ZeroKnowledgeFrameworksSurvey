// Based on https://github.com/arkworks-rs/crypto-primitives/blob/main/src/crh/pedersen/constraints.rs

use ark_crypto_primitives::crh::{CRHScheme, CRHSchemeGadget};
use ark_crypto_primitives::crh::pedersen;
use ark_ed_on_bls12_381::{constraints::EdwardsVar, EdwardsProjective as JubJub, Fq as Fr};
use ark_r1cs_std::{bits::uint8::UInt8, prelude::*};
use ark_relations::{r1cs::{ConstraintSystem}};
use ark_std::rand::RngCore;

type TestCRH = pedersen::CRH<JubJub, Window>;
type TestCRHGadget = pedersen::constraints::CRHGadget<JubJub, EdwardsVar, Window>;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Window;

impl pedersen::Window for Window {
    const WINDOW_SIZE: usize = 127;
    const NUM_WINDOWS: usize = 9;
}

const INPUT_LEN: usize = 128;

pub fn test() {
    println!("start: setup");
    let mut rng = ark_std::test_rng();
    let cs = ConstraintSystem::<Fr>::new_ref();

    let mut input = vec![0u8; INPUT_LEN];
    rng.fill_bytes(&mut input);

    let mut input_var = vec![];
    for byte in input.iter() {
        input_var.push(UInt8::new_witness(cs.clone(), || Ok(byte)).unwrap());
    }

    let param = TestCRH::setup(&mut rng).unwrap();

    let param_var = pedersen::constraints::CRHParametersVar::new_constant(
        ark_relations::ns!(cs, "CRH Parameters"),
        &param,
    )
    .unwrap();
    println!("finish: setup");

    println!("start: hash");
    let result_var = TestCRHGadget::evaluate(&param_var, &input_var).unwrap();
    println!("finish: hash");

    let primitive_result = TestCRH::evaluate(&param, input.as_slice()).unwrap();

    assert_eq!(primitive_result, result_var.value().unwrap());
    assert!(cs.is_satisfied().unwrap());
    println!("test passed");
}
