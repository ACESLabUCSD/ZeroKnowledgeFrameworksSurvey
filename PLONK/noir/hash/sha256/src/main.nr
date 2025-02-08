use dep::std;

fn main(msg: [u8; 64], known_hash: pub [u8; 32]) {
  let calc_hash = std::hash::sha256(msg);
  constrain calc_hash == known_hash;
}
