#![no_main]
sp1_zkvm::entrypoint!(main);

use crypto_bigint::{Integer as CryptoInteger, NonZero, Encoding, U2048, U256, U4096};
use num_bigint::BigUint;
use num_traits::identities::Zero;
use sp1_zkvm::syscalls::syscall_u256x2048_mul;
use sp1_lib::io::hint_slice;
use bytemuck::cast_ref;

pub fn main() {
    let num_iters: u32 = sp1_zkvm::io::read();
    let m: Vec<u8> = sp1_zkvm::io::read();
    let e: u32 = sp1_zkvm::io::read();
    let n: Vec<u8> = sp1_zkvm::io::read();

    let m_u2048 = from_biguint_to_u2048(&BigUint::from_bytes_le(&m));
    let e_u2048 = from_biguint_to_u2048(&BigUint::from(e));
    let n_u2048 = from_biguint_to_u2048(&BigUint::from_bytes_le(&n));
    for _ in 0..num_iters {
        custom_modpow_u2048(&m_u2048, &e_u2048, &n_u2048);
    }

    sp1_zkvm::io::commit(&num_iters);
}

/// Performs modular exponentiation of `base` to the power of `exp` modulo `modulus`.
/// This function takes in U2048 operands and returns the result as a BigUint.
fn custom_modpow_u2048(base: &U2048, exp: &U2048, modulus: &U2048) -> BigUint {
    if *modulus == U2048::ONE {
        return BigUint::zero();
    }

    let mut result = U2048::ONE;
    let modulus_nonzero = NonZero::new(*modulus).unwrap(); // Convert modulus to NonZero
    let mut base = base.rem(&modulus_nonzero);


    let mut exp = *exp;
    while exp > U2048::ZERO {
        if exp.is_odd().into() {
            result = mul_mod_u2048(&result, &base, &modulus_nonzero);
        }
        exp = exp.shr(1);
        base = mul_mod_u2048(&base, &base, &modulus_nonzero);
    }

    let result_biguint = BigUint::from_bytes_le(&result.to_le_bytes());
    result_biguint
}

/// Performs modular multiplication of `a` and `b` with `modulus`.
/// It calculates the quotient and remainder in unconstrained.
fn mul_mod_u2048(a: &U2048, b: &U2048, modulus: &U2048) -> U2048 {
    let prod = mul_u2048(*a, *b);
    sp1_lib::unconstrained! {
        let modulus_u4096 = U4096::from(modulus);
        let modulus_u4096_nonzero = NonZero::new(modulus_u4096).unwrap(); // Convert modulus to NonZero
        let (quotient, result) = prod.div_rem(&modulus_u4096_nonzero);
        let result_bytes = result.to_le_bytes();
        let quotient_bytes = quotient.to_le_bytes();

        hint_slice(&result_bytes[..256]);
        hint_slice(&quotient_bytes[..256]);
    }

    let result_bytes: [u8; 256] = sp1_lib::io::read_vec().try_into().unwrap();
    let quotient_bytes: [u8; 256] = sp1_lib::io::read_vec().try_into().unwrap();

    let q_array = U2048::from_le_slice(&quotient_bytes);
    let result = U2048::from_le_slice(&result_bytes);

    assert!(result > U2048::ZERO && result <= *modulus);
    assert!(prod == mul_u2048(q_array, *modulus).wrapping_add(&U4096::from(&result)));
    result
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
