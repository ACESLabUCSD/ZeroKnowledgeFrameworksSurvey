#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
// #![no_std]  // std support is experimental


use risc0_zkvm::guest::env;
use risc0_zkvm::sha;
use risc0_zkvm::sha::Sha256;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // TODO: Implement your guest code here

    // Load the data from the host
    let msg: Vec<u8> = env::read();

    // Calculate the SHA256 hash digest
    let digest: sha::Digest = *sha::Impl::hash_bytes(&msg);

    env::commit(&digest);
}
