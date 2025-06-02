fn main(msg: [u8; 64], known_hash: pub [u8; 32]) {
  let calc_hash = sha256::digest(msg);
  assert_eq(calc_hash, known_hash);
}
