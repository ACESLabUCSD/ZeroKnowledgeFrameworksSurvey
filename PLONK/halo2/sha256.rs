# extern crate halo2_proofs;
# use halo2_proofs::plonk::Error;
# use std::fmt;
#
# trait Chip: Sized {}
# trait Layouter<C: Chip> {}
const BLOCK_SIZE: usize = 16;
const DIGEST_SIZE: usize = 8;

pub trait Sha256Instructions: Chip {
    /// Variable representing the SHA-256 internal state.
    type State: Clone + fmt::Debug;
    /// Variable representing a 32-bit word of the input block to the SHA-256 compression
    /// function.
    type BlockWord: Copy + fmt::Debug;

    /// Places the SHA-256 IV in the circuit, returning the initial state variable.
    fn initialization_vector(layouter: &mut impl Layouter<Self>) -> Result<Self::State, Error>;

    /// Starting from the given initial state, processes a block of input and returns the
    /// final state.
    fn compress(
        layouter: &mut impl Layouter<Self>,
        initial_state: &Self::State,
        input: [Self::BlockWord; BLOCK_SIZE],
    ) -> Result<Self::State, Error>;

    /// Converts the given state into a message digest.
    fn digest(
        layouter: &mut impl Layouter<Self>,
        state: &Self::State,
    ) -> Result<[Self::BlockWord; DIGEST_SIZE], Error>;
}