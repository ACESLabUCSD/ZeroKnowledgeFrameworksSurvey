use std::time;

use matmul_methods::{MATMUL_ELF, MATMUL_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};

const MATRIX_SIZE: usize = 32; // Specify N for a N x N matrix
const BENCHMARK_ROUNDS: u32 = 10;

fn main() {
    let mut proof_time = time::Duration::ZERO;
    let mut proof_size = 0;
    let mut verify_time = time::Duration::ZERO;

    for _ in 0..BENCHMARK_ROUNDS {
        let a: Vec<u8> = vec![0; MATRIX_SIZE*MATRIX_SIZE];
        let b: Vec<u8> = vec![0; MATRIX_SIZE*MATRIX_SIZE];

        let env = ExecutorEnv::builder()
            .write(&a).unwrap()
            .write(&b).unwrap()
            .build()
            .unwrap();

        let prover = default_prover();

        let mut t: time::Instant;

        t = time::Instant::now();
        let receipt = prover.prove_elf(env,MATMUL_ELF).unwrap();
        proof_time += t.elapsed();

        let bytes = bincode::serialize(&receipt).unwrap();
        proof_size = bytes.len();

        t = time::Instant::now();
        receipt.verify(MATMUL_ID).unwrap();
        verify_time += t.elapsed();

        // Extract journal of receipt (i.e. output c, where c = a * b)
        let c: Vec<u8> = receipt.journal.decode().unwrap();
        println!("Hello, world! Proved that prover knows a and b such that a * b = {:?}", c);
    }

    println!("Proof time: {} us", (proof_time / BENCHMARK_ROUNDS).as_micros());
    println!("Proof size: {} B", proof_size);
    println!("Verify time: {} us", (verify_time / BENCHMARK_ROUNDS).as_micros());
}
