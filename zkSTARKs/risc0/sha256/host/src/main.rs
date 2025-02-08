use std::time;

use rand::{RngCore};
use methods::{SHA256_ELF, SHA256_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use risc0_zkvm::sha;

const HASH_INPUT_LEN: usize = 512; // bits
const BENCHMARK_ROUNDS: u32 = 10;

fn main() {
    let mut rng = rand::thread_rng();

    let mut proof_time = time::Duration::ZERO;
    let mut proof_size = 0;
    let mut verify_time = time::Duration::ZERO;

    for _ in 0..BENCHMARK_ROUNDS {
        // Make a random byte-string of the given length
        let mut message = vec![0u8; HASH_INPUT_LEN / 8];
        rng.fill_bytes(&mut message);

        let env = ExecutorEnv::builder()
            .write(&message).unwrap() // Feed random byte-string as input to guest
            .build().unwrap();

        let prover = default_prover();

        let mut t: time::Instant;

        t = time::Instant::now();
        let receipt = prover.prove_elf(env, SHA256_ELF).unwrap();
        proof_time += t.elapsed();

        let bytes = bincode::serialize(&receipt).unwrap();
        proof_size = bytes.len();

        t = time::Instant::now();
        receipt.verify(SHA256_ID).unwrap();
        verify_time += t.elapsed();

        // Extract journal of receipt (i.e. output h, where h = sha256(m))
        let digest: sha::Digest = receipt.journal.decode().unwrap();
        println!("Hello, world! Proved that prover knows data that hashes to digest {}.", digest);
    }

    println!("Proof time: {} us", (proof_time / BENCHMARK_ROUNDS).as_micros());
    println!("Proof size: {} B", proof_size);
    println!("Verify time: {} us", (verify_time / BENCHMARK_ROUNDS).as_micros());
}
