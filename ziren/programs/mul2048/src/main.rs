#![no_main]
zkm_zkvm::entrypoint!(main);

use crypto_bigint::{Integer as CryptoInteger, Encoding, U2048, U256, U4096};
use num_bigint::BigUint;
use num_traits::identities::Zero;
use zkm_zkvm::syscalls::syscall_u256x2048_mul;
use bytemuck::cast_ref;

pub fn main() {
    let iter: u32 = zkm_zkvm::io::read();
    let a: Vec<u8> = zkm_zkvm::io::read();
    let b: Vec<u8> = zkm_zkvm::io::read();

    let a_u2048 = from_biguint_to_u2048(&BigUint::from_bytes_be(&a));
    let b_u2048 = from_biguint_to_u2048(&BigUint::from_bytes_be(&b));
    for _ in 0..iter {
        mul_u2048(a_u2048, b_u2048);
    }

    zkm_zkvm::io::commit(&iter);
}

/// Performs multiplication of `a` and `b`, which are both U2048,
/// and returns a U4096.
fn mul_u2048(a_array: U2048, b_array: U2048) -> U4096 {
    let mut sum = U4096::ZERO;
    let a_words = a_array.to_words();

    for i in 0..8 {
        let chunk = a_words[i * 8..(i + 1) * 8].try_into().unwrap();
        let a_chunk: U256 = U256::from_words(chunk);
        let mut prod = mul_array(a_chunk, b_array);
        let mut shifted_words = [0u32; 128];
        shifted_words[i * 8..].copy_from_slice(&prod.to_words()[..(128 - 8 * i)]);
        let shifted_prod = U4096::from_words(shifted_words);
        sum = sum.wrapping_add(&shifted_prod);
    }

    sum
}

/// Performs multiplication of `a` a U256 and `b` which is a U2048.
fn mul_array(a: U256, b_array: U2048) -> U4096 {
    let mut result_words = [0u32; 128];
    let result_ptr = result_words.as_mut_ptr();
    unsafe {
        syscall_u256x2048_mul(
            cast_ref(&a.to_words()),
            cast_ref(&b_array.to_words()),
            result_ptr as *mut [u32; 64],
            result_ptr.add(64) as *mut [u32; 8],
        );
    }

    U4096::from_words(result_words)
}

/// Converts a BigUint to a U2048.
fn from_biguint_to_u2048(value: &BigUint) -> U2048 {
    let mut padded_bytes = [0u8; 256];
    let a_bytes = value.to_bytes_le();
    for (i, &byte) in a_bytes.iter().enumerate() {
        if i >= 256 { break; }
        padded_bytes[i] = byte;
    }

    U2048::from_le_slice(&padded_bytes)
}


