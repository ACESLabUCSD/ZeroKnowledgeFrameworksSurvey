mod demo_sha256;
mod demo_pedersen;
mod demo_poseidon;

fn main() {
    println!("start test: SHA256 hash");
    demo_sha256::test();
    println!("finish test: SHA256 hash");

    println!("start test: Pedersen hash");
    demo_pedersen::test();
    println!("finish test: Pedersen hash");

    println!("start test: Poseidon hash");
    demo_poseidon::test_2_to_one();
    println!("finish test: Poseidon hash");
}
