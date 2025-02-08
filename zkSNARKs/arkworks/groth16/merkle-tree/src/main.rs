// Based on https://github.com/arkworks-rs/r1cs-tutorial/tree/main/merkle-tree-example

use ark_crypto_primitives::crh::{TwoToOneCRH, CRH};
use ark_crypto_primitives::crh::constraints::{CRHGadget, TwoToOneCRHGadget};
use ark_crypto_primitives::crh::injective_map::constraints::{
    PedersenCRHCompressorGadget, TECompressorGadget,
};
use ark_crypto_primitives::crh::{
    injective_map::{PedersenCRHCompressor, TECompressor},
    pedersen,
};
use ark_ed_on_bls12_381::{constraints::EdwardsVar, EdwardsProjective};
use ark_crypto_primitives::merkle_tree::constraints::PathVar;
use ark_crypto_primitives::merkle_tree::{Config, MerkleTree, Path};
use ark_r1cs_std::prelude::*;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};

pub type TwoToOneHash = PedersenCRHCompressor<EdwardsProjective, TECompressor, TwoToOneWindow>;
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TwoToOneWindow;

// `WINDOW_SIZE * NUM_WINDOWS` = enough for hashing two outputs.
impl pedersen::Window for TwoToOneWindow {
    const WINDOW_SIZE: usize = 4;
    const NUM_WINDOWS: usize = 128;
}

pub type LeafHash = PedersenCRHCompressor<EdwardsProjective, TECompressor, LeafWindow>;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct LeafWindow;

// `WINDOW_SIZE * NUM_WINDOWS` = enough for hashing two outputs.
impl pedersen::Window for LeafWindow {
    const WINDOW_SIZE: usize = 4;
    const NUM_WINDOWS: usize = 144;
}

pub type TwoToOneHashGadget = PedersenCRHCompressorGadget<
    EdwardsProjective,
    TECompressor,
    TwoToOneWindow,
    EdwardsVar,
    TECompressorGadget,
>;

pub type LeafHashGadget = PedersenCRHCompressorGadget<
    EdwardsProjective,
    TECompressor,
    LeafWindow,
    EdwardsVar,
    TECompressorGadget,
>;

pub type LeafHashParamsVar = <LeafHashGadget as CRHGadget<LeafHash, ConstraintF>>::ParametersVar;
pub type TwoToOneHashParamsVar =
    <TwoToOneHashGadget as TwoToOneCRHGadget<TwoToOneHash, ConstraintF>>::ParametersVar;

pub type ConstraintF = ark_ed_on_bls12_381::Fq;

#[derive(Clone)]
pub struct MerkleConfig;
impl Config for MerkleConfig {
    // Our Merkle tree relies on two hashes: one to hash leaves, and one to hash pairs
    // of internal nodes.
    type LeafHash = LeafHash;
    type TwoToOneHash = TwoToOneHash;
}

/// A Merkle tree containing account information.
pub type SimpleMerkleTree = MerkleTree<MerkleConfig>;
/// The root of the account Merkle tree.
pub type Root = <TwoToOneHash as TwoToOneCRH>::Output;
/// A membership proof for a given account.
pub type SimplePath = Path<MerkleConfig>;

// (You don't need to worry about what's going on in the next two type definitions,
// just know that these are types that you can use.)

/// The R1CS equivalent of the the Merkle tree root.
pub type RootVar = <TwoToOneHashGadget as TwoToOneCRHGadget<TwoToOneHash, ConstraintF>>::OutputVar;

/// The R1CS equivalent of the the Merkle tree path.
pub type SimplePathVar =
    PathVar<crate::MerkleConfig, LeafHashGadget, TwoToOneHashGadget, ConstraintF>;

////////////////////////////////////////////////////////////////////////////////

pub struct MerkleTreeVerificationOneLeaf {
    // These are constants that will be embedded into the circuit
    pub leaf_crh_params: <LeafHash as CRH>::Parameters,
    pub two_to_one_crh_params: <TwoToOneHash as TwoToOneCRH>::Parameters,

    // These are the public inputs to the circuit.
    pub root: Root,
    pub leaf: u8,

    // This is the private witness to the circuit.
    pub authentication_path: Option<SimplePath>,
}

impl ConstraintSynthesizer<ConstraintF> for MerkleTreeVerificationOneLeaf {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        // First, we allocate the public inputs
        let root = RootVar::new_input(ark_relations::ns!(cs, "root_var"), || Ok(&self.root))?;

        let leaf = UInt8::new_input(ark_relations::ns!(cs, "leaf_var"), || Ok(&self.leaf))?;

        // Then, we allocate the public parameters as constants:
        let leaf_crh_params = LeafHashParamsVar::new_constant(cs.clone(), &self.leaf_crh_params)?;
        let two_to_one_crh_params =
            TwoToOneHashParamsVar::new_constant(cs.clone(), &self.two_to_one_crh_params)?;

        // Finally, we allocate our path as a private witness variable:
        let path = SimplePathVar::new_witness(ark_relations::ns!(cs, "path_var"), || {
            Ok(self.authentication_path.as_ref().unwrap())
        })?;

        let leaf_bytes = vec![leaf; 1];

        // Now, we have to check membership.
        // This follows the API in  https://github.com/arkworks-rs/crypto-primitives/blob/6be606259eab0aec010015e2cfd45e4f134cd9bf/src/merkle_tree/constraints.rs#L135

        let is_member = path.verify_membership(
            &leaf_crh_params,
            &two_to_one_crh_params,
            &root,
            &leaf_bytes.as_slice(),
        )?;

        is_member.enforce_equal(&Boolean::TRUE)?;

        Ok(())
    }
}

fn test_correctness_one_leaf() {
    use ark_relations::r1cs::{ConstraintLayer, ConstraintSystem, TracingMode};
    use tracing_subscriber::layer::SubscriberExt;

    // Let's set up an RNG for use within tests. Note that this is *not* safe
    // for any production use.
    let mut rng = ark_std::test_rng();

    // First, let's sample the public parameters for the hash functions:
    let leaf_crh_params = <LeafHash as CRH>::setup(&mut rng).unwrap();
    let two_to_one_crh_params = <TwoToOneHash as TwoToOneCRH>::setup(&mut rng).unwrap();

    // Next, let's construct our tree.
    // This follows the API in https://github.com/arkworks-rs/crypto-primitives/blob/6be606259eab0aec010015e2cfd45e4f134cd9bf/src/merkle_tree/mod.rs#L156
    let tree = crate::SimpleMerkleTree::new(
        &leaf_crh_params,
        &two_to_one_crh_params,
        &[1u8, 2u8, 3u8, 10u8, 9u8, 17u8, 70u8, 45u8], // the i-th entry is the i-th leaf.
    )
    .unwrap();

    // Now, let's try to generate a membership proof for the 5th item, i.e. 9.
    let proof = tree.generate_proof(4).unwrap(); // we're 0-indexing!
                                                 // This should be a proof for the membership of a leaf with value 9. Let's check that!

    // First, let's get the root we want to verify against:
    let root = tree.root();

    let circuit = MerkleTreeVerificationOneLeaf {
        // constants
        leaf_crh_params,
        two_to_one_crh_params,

        // public inputs
        root,
        leaf: 9u8,

        // witness
        authentication_path: Some(proof),
    };
    // First, some boilerplat that helps with debugging
    let mut layer = ConstraintLayer::default();
    layer.mode = TracingMode::OnlyConstraints;
    let subscriber = tracing_subscriber::Registry::default().with(layer);
    let _guard = tracing::subscriber::set_default(subscriber);

    // Next, let's make the circuit!
    let cs = ConstraintSystem::new_ref();
    circuit.generate_constraints(cs.clone()).unwrap();
    // Let's check whether the constraint system is satisfied
    let is_satisfied = cs.is_satisfied().unwrap();
    if !is_satisfied {
        // If it isn't, find out the offending constraint.
        println!("{:?}", cs.which_is_unsatisfied());
    }
    assert!(is_satisfied);
}

fn test_soundness_one_leaf() {
    use ark_relations::r1cs::{ConstraintLayer, ConstraintSystem, TracingMode};
    use tracing_subscriber::layer::SubscriberExt;

    // Let's set up an RNG for use within tests. Note that this is *not* safe
    // for any production use.
    let mut rng = ark_std::test_rng();

    // First, let's sample the public parameters for the hash functions:
    let leaf_crh_params = <LeafHash as CRH>::setup(&mut rng).unwrap();
    let two_to_one_crh_params = <TwoToOneHash as TwoToOneCRH>::setup(&mut rng).unwrap();

    // Next, let's construct our tree.
    // This follows the API in https://github.com/arkworks-rs/crypto-primitives/blob/6be606259eab0aec010015e2cfd45e4f134cd9bf/src/merkle_tree/mod.rs#L156
    let tree = crate::SimpleMerkleTree::new(
        &leaf_crh_params,
        &two_to_one_crh_params,
        &[1u8, 2u8, 3u8, 10u8, 9u8, 17u8, 70u8, 45u8], // the i-th entry is the i-th leaf.
    )
    .unwrap();

    // We just mutate the first leaf
    let second_tree = crate::SimpleMerkleTree::new(
        &leaf_crh_params,
        &two_to_one_crh_params,
        &[4u8, 2u8, 3u8, 10u8, 9u8, 17u8, 70u8, 45u8], // the i-th entry is the i-th leaf.
    )
    .unwrap();

    // Now, let's try to generate a membership proof for the 5th item, i.e. 9.
    let proof = tree.generate_proof(4).unwrap(); // we're 0-indexing!

    // But, let's get the root we want to verify against:
    let wrong_root = second_tree.root();

    let circuit = MerkleTreeVerificationOneLeaf {
        // constants
        leaf_crh_params,
        two_to_one_crh_params,

        // public inputs
        root: wrong_root,
        leaf: 9u8,

        // witness
        authentication_path: Some(proof),
    };
    // First, some boilerplate that helps with debugging
    let mut layer = ConstraintLayer::default();
    layer.mode = TracingMode::OnlyConstraints;
    let subscriber = tracing_subscriber::Registry::default().with(layer);
    let _guard = tracing::subscriber::set_default(subscriber);

    // Next, let's make the constraint system!
    let cs = ConstraintSystem::new_ref();
    circuit.generate_constraints(cs.clone()).unwrap();
    // Let's check whether the constraint system is satisfied
    let is_satisfied = cs.is_satisfied().unwrap();
    // We expect this to fail!
    assert!(!is_satisfied);
}

pub struct MerkleTreeVerificationAllLeaf {
    // These are constants that will be embedded into the circuit
    pub leaf_crh_params: <LeafHash as CRH>::Parameters,
    pub two_to_one_crh_params: <TwoToOneHash as TwoToOneCRH>::Parameters,

    // These are the public inputs to the circuit.
    pub root: Root,
    pub leaves: Vec<u8>,

    // This is the private witness to the circuit.
    pub authentication_paths: Vec<Option<SimplePath>>,
}

impl ConstraintSynthesizer<ConstraintF> for MerkleTreeVerificationAllLeaf {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        // First, we allocate the public inputs
        let root = RootVar::new_input(ark_relations::ns!(cs, "root_var"), || Ok(&self.root))?;

        let leaves: Vec<_> = self.leaves.iter().map(
            |leaf| UInt8::new_input(ark_relations::ns!(cs, "leaf_var"), || Ok(leaf)).unwrap()
        ).collect();

        // Then, we allocate the public parameters as constants:
        let leaf_crh_params = LeafHashParamsVar::new_constant(cs.clone(), &self.leaf_crh_params)?;
        let two_to_one_crh_params =
            TwoToOneHashParamsVar::new_constant(cs.clone(), &self.two_to_one_crh_params)?;

        // Finally, we allocate our paths as private witness variables:
        let paths: Vec<_> = self.authentication_paths.iter().map(
            |path| SimplePathVar::new_witness(ark_relations::ns!(cs, "path_var"), || {
                Ok(path.as_ref().unwrap())
            }).unwrap()
        ).collect();

        for (leaf, path) in std::iter::zip(leaves, paths) {
            let leaf_bytes = vec![leaf; 1];

            // Now, we have to check membership.
            // This follows the API in https://github.com/arkworks-rs/crypto-primitives/blob/6be606259eab0aec010015e2cfd45e4f134cd9bf/src/merkle_tree/constraints.rs#L135

            let is_member = path.verify_membership(
                &leaf_crh_params,
                &two_to_one_crh_params,
                &root,
                &leaf_bytes.as_slice(),
            )?;

            is_member.enforce_equal(&Boolean::TRUE)?;
        }

        Ok(())
    }
}

fn test_correctness_all_leaf() {
    use ark_relations::r1cs::{ConstraintLayer, ConstraintSystem, TracingMode};
    use tracing_subscriber::layer::SubscriberExt;

    // Let's set up an RNG for use within tests. Note that this is *not* safe
    // for any production use.
    let mut rng = ark_std::test_rng();

    // First, let's sample the public parameters for the hash functions:
    let leaf_crh_params = <LeafHash as CRH>::setup(&mut rng).unwrap();
    let two_to_one_crh_params = <TwoToOneHash as TwoToOneCRH>::setup(&mut rng).unwrap();

    // Next, let's construct our tree.
    // This follows the API in https://github.com/arkworks-rs/crypto-primitives/blob/6be606259eab0aec010015e2cfd45e4f134cd9bf/src/merkle_tree/mod.rs#L156
    let leaves = vec![1u8, 2u8, 3u8, 10u8, 9u8, 17u8, 70u8, 45u8];
    let tree = crate::SimpleMerkleTree::new(
        &leaf_crh_params,
        &two_to_one_crh_params,
        &leaves, // the i-th entry is the i-th leaf.
    )
    .unwrap();

    // Now, let's try to generate a membership proof for each leaf
    let proofs: Vec<_> = (0..leaves.len()).map(
        |idx| tree.generate_proof(idx).unwrap()
    ).collect();

    // First, let's get the root we want to verify against:
    let root = tree.root();

    let circuit = MerkleTreeVerificationAllLeaf {
        // constants
        leaf_crh_params,
        two_to_one_crh_params,

        // public inputs
        root,
        leaves,

        // witness
        authentication_paths: proofs.clone().iter().map(|x| Some(x.clone())).collect(),
    };
    // First, some boilerplat that helps with debugging
    let mut layer = ConstraintLayer::default();
    layer.mode = TracingMode::OnlyConstraints;
    let subscriber = tracing_subscriber::Registry::default().with(layer);
    let _guard = tracing::subscriber::set_default(subscriber);

    // Next, let's make the circuit!
    let cs = ConstraintSystem::new_ref();
    circuit.generate_constraints(cs.clone()).unwrap();
    // Let's check whether the constraint system is satisfied
    let is_satisfied = cs.is_satisfied().unwrap();
    if !is_satisfied {
        // If it isn't, find out the offending constraint.
        println!("{:?}", cs.which_is_unsatisfied());
    }
    assert!(is_satisfied);
}

fn main() {
    test_correctness_one_leaf();
    test_soundness_one_leaf();

    test_correctness_all_leaf();
}
