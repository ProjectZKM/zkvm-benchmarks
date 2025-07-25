#![no_std]
#![no_main]
extern crate alloc;

use zkm_zkvm::syscalls::syscall_poseidon2_permute;

zkm_zkvm::entrypoint!(main);

pub fn main() {
    let input: [u32; 16] = zkm_zkvm::io::read();
    let num_iters: u32 = zkm_zkvm::io::read();

    let mut state = input;
    for _ in 0..num_iters {
        unsafe { syscall_poseidon2_permute(&mut state) };
    }

    zkm_zkvm::io::commit::<[u32; 16]>(&state);
}
