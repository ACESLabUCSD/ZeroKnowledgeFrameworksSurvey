#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
// #![no_std]  // std support is experimental


use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

const MATRIX_SIZE: usize = 32; // Specify N for a N x N matrix

pub fn main() {
    // TODO: Implement your guest code here
    // Load the first number from the host
    let a: Vec<u8> = env::read();
    // Load the second number from the host
    let b: Vec<u8> = env::read();

    // Check input size
    if MATRIX_SIZE*MATRIX_SIZE != a.len() {
        panic!("The input matrices must be of size {} x {}", MATRIX_SIZE, MATRIX_SIZE);
    }

    // TODO: Verify that neither a nor b is a nontrivial factor

    // Compute the product (while being careful with integer overflow?)
    let n: usize = MATRIX_SIZE;
    let mut product: Vec<u8> = vec![0; n*n];
    for i in 0..n{
        for j in 0..n{
            for k in 0..n{
                // println!("c[{},{}]", i, j);
                product[i*n + j] += a[i*n + k] * b[k*n + j];
            }
        }
    }

    env::commit(&product);
}
