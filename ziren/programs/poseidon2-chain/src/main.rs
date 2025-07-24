#![no_std]
#![no_main]
extern crate alloc;
use alloc::vec::Vec;

zkm_zkvm::entrypoint!(main);

pub fn main() {
    let input: [u8; 32] = zkm_zkvm::io::read();
    let num_iters: u32 = zkm_zkvm::io::read();
    let mut hash = input;
    for _ in 0..num_iters {
        hash = poseidon2::poseidon2(&hash);
    }

    zkm_zkvm::io::commit::<[u8; 32]>(&hash.into());
}
