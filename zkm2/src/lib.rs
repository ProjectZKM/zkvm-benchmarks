// use num_bigint::BigUint;
// use num_traits::Num;
// use std::str::FromStr;
use std::time::{Duration, Instant};
use zkm_build::include_elf;
use zkm_sdk::{ProverClient, ZKMStdin};
use utils::size;

mod tendermint;
pub use tendermint::bench_tendermint;

const FIBONACCI_ELF: &[u8] = include_elf!("fibonacci");
const SHA2_ELF: &[u8] = include_elf!("sha2-bench");
const SHA2_CHAIN_ELF: &[u8] = include_elf!("sha2-chain");
const SHA3_CHAIN_ELF: &[u8] = include_elf!("sha3-chain");
const SHA3_ELF: &[u8] = include_elf!("sha3-bench");
const BIGMEM_ELF: &[u8] = include_elf!("bigmem");
const MODPOW_ELF: &[u8] = include_elf!("modpow");

pub fn init_logger() {
    std::env::set_var("RUST_LOG", "info");
    zkm_core_machine::utils::setup_logger();
}

pub fn benchmark_with_shard_size(func: fn(u32) -> (Duration, usize, u64), iters: &[u32], shard_sizes: &[usize], file_name: &str, input_name: &str) {
    assert_eq!(iters.len(), shard_sizes.len());
    let mut info = Vec::new();
    for bench_i in 0..iters.len() {
        println!("benchmark_with_shard_size, bench_i: {}, shard_size: {}", bench_i, shard_sizes[bench_i]);
        std::env::set_var("SHARD_SIZE", format!("{}", shard_sizes[bench_i]));
        let duration_and_size_and_cycles = func(iters[bench_i]);
        info.push(duration_and_size_and_cycles);
        println!(
            "benchmark_with_shard_size end, duration: {:?}, shard_size: {}",
            duration_and_size_and_cycles.0.as_secs_f64(), duration_and_size_and_cycles.1,
        );
    }
    utils::write_csv_v2(file_name, input_name, iters, &info);
}

pub fn benchmark_sha2_chain(iters: u32) -> (Duration, usize, u64) {
    let client = ProverClient::cpu();
    let (pk, vk) = client.setup(SHA2_CHAIN_ELF);

    let mut stdin = ZKMStdin::new();
    let input = [5u8; 32];
    stdin.write(&input);
    stdin.write(&iters);

    println!("benchmark_sha2_chain start, iters: {}", iters);
    let start = Instant::now();
    let proof = client.prove(&pk, stdin.clone()).run().unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!("benchmark_sha2_chain end, duration: {:?}", duration.as_secs_f64());

    client.verify(&proof, &vk).expect("verification failed");

    // Execute the program using the `ProverClient.execute` method, without generating a proof.
    let (_, report) = client.execute(SHA2_CHAIN_ELF, stdin).run().unwrap();
    println!("executed program with {} cycles", report.total_instruction_count());

    (duration, size(&proof), report.total_instruction_count())
}

pub fn benchmark_sha3_chain(iters: u32) -> (Duration, usize, u64) {
    let client = ProverClient::cpu();
    let (pk, vk) = client.setup(SHA3_CHAIN_ELF);

    let mut stdin = ZKMStdin::new();
    let input = [5u8; 32];
    stdin.write(&input);
    stdin.write(&iters);

    println!("benchmark_sha3_chain start, iters: {}", iters);
    let start = Instant::now();
    let proof = client.prove(&pk, stdin.clone()).run().unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!("benchmark_sha3 end, duration: {:?}", duration.as_secs_f64());

    client.verify(&proof, &vk).expect("verification failed");

    // Execute the program using the `ProverClient.execute` method, without generating a proof.
    let (_, report) = client.execute(SHA3_CHAIN_ELF, stdin).run().unwrap();
    println!("executed program with {} cycles", report.total_instruction_count());

    (duration, size(&proof), report.total_instruction_count())
}

pub fn benchmark_sha2(num_bytes: usize) -> (Duration, usize, u64) {
    let client = ProverClient::cpu();
    let (pk, vk) = client.setup(SHA2_ELF);

    let mut stdin = ZKMStdin::new();
    let input = vec![5u8; num_bytes];
    stdin.write(&input);

    println!("benchmark_sha2 start, num_bytes: {}", num_bytes);
    let start = Instant::now();
    let proof = client.prove(&pk, stdin.clone()).run().unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!("benchmark_sha2 end, duration: {:?}", duration.as_secs_f64());

    client.verify(&proof, &vk).expect("verification failed");

    // Execute the program using the `ProverClient.execute` method, without generating a proof.
    let (_, report) = client.execute(SHA2_ELF, stdin).run().unwrap();
    println!("executed program with {} cycles", report.total_instruction_count());

    (duration, size(&proof), report.total_instruction_count())
}

pub fn benchmark_sha3(num_bytes: usize) -> (Duration, usize, u64) {
    let client = ProverClient::cpu();
    let (pk, vk) = client.setup(SHA3_ELF);

    let mut stdin = ZKMStdin::new();
    let input = vec![5u8; num_bytes];
    stdin.write(&input);

    println!("benchmark_sha3 start, num_bytes: {}", num_bytes);
    let start = Instant::now();
    let proof = client.prove(&pk, stdin.clone()).run().unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!("benchmark_sha3 end, duration: {:?}", duration.as_secs_f64());

    client.verify(&proof, &vk).expect("verification failed");

    // Execute the program using the `ProverClient.execute` method, without generating a proof.
    let (_, report) = client.execute(SHA3_ELF, stdin).run().unwrap();
    println!("executed program with {} cycles", report.total_instruction_count());

    (duration, size(&proof), report.total_instruction_count())
}

pub fn bench_fibonacci(n: u32) -> (Duration, usize, u64) {
    let client = ProverClient::cpu();
    let (pk, vk) = client.setup(FIBONACCI_ELF);

    let mut stdin = ZKMStdin::new();
    stdin.write(&n);

    println!("benchmark_fibonacci start, n: {}", n);
    let start = Instant::now();
    let proof = client.prove(&pk, stdin.clone()).run().unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!("benchmark_fibonacc end, duration: {:?}", duration.as_secs_f64());

    client.verify(&proof, &vk).expect("verification failed");

    // Execute the program using the `ProverClient.execute` method, without generating a proof.
    let (_, report) = client.execute(FIBONACCI_ELF, stdin).run().unwrap();
    println!("executed program with {} cycles", report.total_instruction_count());

    (duration, size(&proof), report.total_instruction_count())
}

pub fn bench_bigmem(value: u32) -> (Duration, usize, u64) {
    let client = ProverClient::cpu();
    let (pk, vk) = client.setup(BIGMEM_ELF);

    let mut stdin = ZKMStdin::new();
    stdin.write(&value);

    println!("benchmark_bigmem start, value: {}", value);
    let start = Instant::now();
    let proof = client.prove(&pk, stdin.clone()).run().unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!("benchmark_bigmem end, duration: {:?}", duration.as_secs_f64());

    client.verify(&proof, &vk).expect("verification failed");

    // Execute the program using the `ProverClient.execute` method, without generating a proof.
    let (_, report) = client.execute(BIGMEM_ELF, stdin).run().unwrap();
    println!("executed program with {} cycles", report.total_instruction_count());

    (duration, size(&proof), report.total_instruction_count())
}

pub fn benchmark_modpow(iters: u32) -> (Duration, usize, u64) {
    let client = ProverClient::cpu();
    let (pk, vk) = client.setup(MODPOW_ELF);

    let mut stdin = ZKMStdin::new();
    stdin.write(&iters);
    let m = hex::decode(
        "60908e3e36666b77de03caf7807ebe62e78d016aa5695ff5bc10b4fbbbe1f9cc"
    )
        .unwrap();
    let e = 65537u32;
    let n = hex::decode("c5529f21f0afe6df78e83aab07b66c11ed9203af47a8ab9fdda4a83b4ae767720b833d2e150fcb4a4aec2776d0aa9762a3955d402b0c2665d3c4aa5db002656b65c75712eef82289d92bd6a3fba04d846e3680d1f9c0598a6717f07ae65400feb9d62156ecb37e0ef0c781299e300e268d825205ffad8892e267e63083348de4670907a8e23d4a03bc3a34496abb923fdf6181126cb073cf7a41620be431c7e1dc65e3d80a62fd76d04f8d011435529c7d683fc9f7c766c4527d3082b7dd2e5254876e1c8b296f41618c92cbb359b54df35010caa84286c35d7bf32c2fefd11c655fa48390c35d54274454a0ff749f8951fb23ee79a01e51a052716df0bc44db").unwrap();
    stdin.write(&m);
    stdin.write(&e);
    stdin.write(&n);

    // Execute the program using the `ProverClient.execute` method, without generating a proof.
    let (_, report) = client.execute(MODPOW_ELF, stdin.clone()).run().unwrap();
    println!(
        "executed program with {} cycles",
        report.total_instruction_count()
    );

    println!("benchmark_modpow start, iters: {}", iters);
    let start = Instant::now();
    let proof = client.prove(&pk, stdin.clone()).run().unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!(
        "benchmark_modpow end, duration: {:?}",
        duration.as_secs_f64()
    );

    client.verify(&proof, &vk).expect("verification failed");

    (duration, size(&proof), report.total_instruction_count())
}