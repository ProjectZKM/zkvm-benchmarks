#![no_main]

zkm2_zkvm::entrypoint!(main);
// use zkm2_zkvm::lib::hasher::Hasher;
use zkm2_zkvm::lib::keccak256::keccak256;
pub fn main() {
    let input: [u8; 32] = zkm2_zkvm::io::read();
    let num_iters: u32 = zkm2_zkvm::io::read();
    let mut hash = input;
    for _ in 0..num_iters {
        hash = keccak256(&input.as_slice());
    }

    zkm2_zkvm::io::commit::<[u8; 32]>(&hash.into());
}
// fn keccak256<T: AsRef<[u8]>>(bytes: T) -> [u8; 32] {
//     let mut output = [0u8; 32];
//     let mut hasher = zkm2_zkvm::lib::keccak::Keccak::v256();
//     hasher.update(bytes.as_ref());
//     hasher.finalize(&mut output);
//     output
// }
