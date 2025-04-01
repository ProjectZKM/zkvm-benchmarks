#![no_main]

use sha3::{Digest, Keccak256};

zkm2_zkvm::entrypoint!(main);
use zkm2_zkvm::lib::sha3::sha3_256;
pub fn main() {
    let input: [u8; 32] = zkm2_zkvm::io::read();
    let num_iters: u32 = zkm2_zkvm::io::read();
    let mut hash = input;
    for _ in 0..num_iters {
        hash = sha3_256(&input.as_slice());
    }

    zkm2_zkvm::io::commit::<[u8; 32]>(&hash.into());
}
