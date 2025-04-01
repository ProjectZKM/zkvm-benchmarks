#![no_main]

extern crate alloc;

zkm2_zkvm::entrypoint!(main);
use zkm2_zkvm::lib::sha3::sha3_256;
pub fn main() {
    let input: Vec<u8> = zkm2_zkvm::io::read();
    let result = sha3_256(&input.as_slice());
    zkm2_zkvm::io::commit::<[u8; 32]>(&result.into());
}
