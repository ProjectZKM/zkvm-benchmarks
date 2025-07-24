#![no_std]
#![no_main]
extern crate alloc;
use alloc::vec::Vec;

zkm_zkvm::entrypoint!(main);

pub fn main() {
    let input: Vec<u8> = zkm_zkvm::io::read();

    let result = poseidon2::poseidon2(&input);

    zkm_zkvm::io::commit::<[u8; 32]>(&result.into());
}
