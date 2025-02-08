// Based on https://github.com/arkworks-rs/crypto-primitives/blob/main/src/crh/poseidon/constraints.rs

use ark_crypto_primitives::crh::poseidon::constraints::{CRHGadget, CRHParametersVar, TwoToOneCRHGadget};
use ark_crypto_primitives::crh::poseidon::{TwoToOneCRH, CRH};
use ark_crypto_primitives::crh::{CRHScheme, CRHSchemeGadget};
use ark_crypto_primitives::crh::{TwoToOneCRHScheme, TwoToOneCRHSchemeGadget};
use ark_crypto_primitives::sponge::poseidon::PoseidonConfig;
use ark_bls12_377::Fr;
use ark_r1cs_std::alloc::AllocVar;
use ark_r1cs_std::{
    fields::fp::{AllocatedFp, FpVar},
    R1CSVar,
};
use ark_relations::r1cs::ConstraintSystem;
use ark_std::UniformRand;

pub fn test_2_to_one() {
    println!("start: setup");
    let mut test_rng = ark_std::test_rng();

    // The following way of generating the MDS matrix is incorrect
    // and is only for test purposes.

    let mut mds = vec![vec![]; 3];
    for i in 0..3 {
        for _ in 0..3 {
            mds[i].push(Fr::rand(&mut test_rng));
        }
    }

    let mut ark = vec![vec![]; 8 + 24];
    for i in 0..8 + 24 {
        for _ in 0..3 {
            ark[i].push(Fr::rand(&mut test_rng));
        }
    }

    let mut test_a = Vec::new();
    let mut test_b = Vec::new();
    for _ in 0..3 {
        test_a.push(Fr::rand(&mut test_rng));
        test_b.push(Fr::rand(&mut test_rng));
    }

    let params = PoseidonConfig::<Fr>::new(8, 24, 31, mds, ark, 2, 1);

    let cs = ConstraintSystem::<Fr>::new_ref();

    let mut test_a_g = Vec::new();
    let mut test_b_g = Vec::new();

    for elem in test_a.iter() {
        test_a_g.push(FpVar::Var(
            AllocatedFp::<Fr>::new_witness(cs.clone(), || Ok(elem)).unwrap(),
        ));
    }
    for elem in test_b.iter() {
        test_b_g.push(FpVar::Var(
            AllocatedFp::<Fr>::new_witness(cs.clone(), || Ok(elem)).unwrap(),
        ));
    }

    let params_g = CRHParametersVar::<Fr>::new_constant(cs, &params).unwrap();
    println!("finish: setup");

    println!("start: hash");
    let crh_a_g = CRHGadget::<Fr>::evaluate(&params_g, &test_a_g).unwrap();
    let crh_b_g = CRHGadget::<Fr>::evaluate(&params_g, &test_b_g).unwrap();
    let crh_g = TwoToOneCRHGadget::<Fr>::compress(&params_g, &crh_a_g, &crh_b_g).unwrap();
    println!("finish: hash");

    let crh_a = CRH::<Fr>::evaluate(&params, test_a.clone()).unwrap();
    let crh_b = CRH::<Fr>::evaluate(&params, test_b.clone()).unwrap();
    let crh = TwoToOneCRH::<Fr>::compress(&params, crh_a, crh_b).unwrap();

    assert_eq!(crh_a, crh_a_g.value().unwrap());
    assert_eq!(crh_b, crh_b_g.value().unwrap());
    assert_eq!(crh, crh_g.value().unwrap());
}
