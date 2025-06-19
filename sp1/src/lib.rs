use std::time::{Duration, Instant};

use sp1_build::include_elf;
use sp1_sdk::{EnvProver, SP1Stdin};
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
const MUL2048_ELF: &[u8] = include_elf!("mul2048");

pub fn init_logger() {
    std::env::set_var("RUST_LOG", "info");
    sp1_core_machine::utils::setup_logger();
}

pub fn benchmark_with_shard_size(
    func: fn(u32) -> (Duration, usize, u64),
    iters: &[u32],
    shard_sizes: &[usize],
    file_name: &str,
    input_name: &str,
) {
    assert_eq!(iters.len(), shard_sizes.len());
    let mut info = Vec::new();
    for bench_i in 0..iters.len() {
        println!(
            "benchmark_with_shard_size start, bench_i: {}, shard_size: {}",
            bench_i, shard_sizes[bench_i]
        );
        std::env::set_var("SHARD_SIZE", format!("{}", shard_sizes[bench_i]));
        let duration_and_size_and_cycles = func(iters[bench_i]);
        info.push(duration_and_size_and_cycles);
        println!(
            "benchmark_with_shard_size end, duration: {:?}, shard_size: {}",
            duration_and_size_and_cycles.0.as_secs_f64(),
            duration_and_size_and_cycles.1,
        );
    }
    utils::write_csv_v2(file_name, input_name, iters, &info);
}

pub fn benchmark_sha2_chain(iters: u32) -> (Duration, usize, u64) {
    let client = EnvProver::new();
    let (pk, vk) = client.setup(SHA2_CHAIN_ELF);

    let mut stdin = SP1Stdin::new();
    let input = [5u8; 32];
    stdin.write(&input);
    stdin.write(&iters);

    println!("benchmark_sha2_chain start, iters: {}", iters);
    let start = Instant::now();
    let proof = client.prove(&pk, &stdin).run().unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!(
        "benchmark_sha2_chain end, duration: {:?}",
        duration.as_secs_f64()
    );

    client.verify(&proof, &vk).expect("verification failed");

    // Execute the program using the `ProverClient.execute` method, without generating a proof.
    let (_, report) = client.execute(SHA2_CHAIN_ELF, &stdin).run().unwrap();
    println!(
        "executed program with {} cycles",
        report.total_instruction_count()
    );

    (duration, size(&proof), report.total_instruction_count())
}

pub fn benchmark_sha3_chain(iters: u32) -> (Duration, usize, u64) {
    let client = EnvProver::new();
    let (pk, vk) = client.setup(SHA3_CHAIN_ELF);

    let mut stdin = SP1Stdin::new();
    let input = [5u8; 32];
    stdin.write(&input);
    stdin.write(&iters);

    println!("benchmark_sha3_chain start, iters: {}", iters);
    let start = Instant::now();
    let proof = client.prove(&pk, &stdin).run().unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!(
        "benchmark_sha3_chain end, duration: {:?}",
        duration.as_secs_f64()
    );

    client.verify(&proof, &vk).expect("verification failed");

    // Execute the program using the `ProverClient.execute` method, without generating a proof.
    let (_, report) = client.execute(SHA3_CHAIN_ELF, &stdin).run().unwrap();
    println!(
        "executed program with {} cycles",
        report.total_instruction_count()
    );

    (duration, size(&proof), report.total_instruction_count())
}

pub fn benchmark_sha2(num_bytes: usize) -> (Duration, usize, u64) {
    let client = EnvProver::new();
    let (pk, vk) = client.setup(SHA2_ELF);

    let mut stdin = SP1Stdin::new();
    let input = vec![5u8; num_bytes];
    stdin.write(&input);

    println!("benchmark_sha2 start, num_bytes: {}", num_bytes);
    let start = Instant::now();
    let proof = client.prove(&pk, &stdin).run().unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!("benchmark_sha2 end, duration: {:?}", duration.as_secs_f64());

    client.verify(&proof, &vk).expect("verification failed");

    // Execute the program using the `ProverClient.execute` method, without generating a proof.
    let (_, report) = client.execute(SHA2_ELF, &stdin).run().unwrap();
    println!(
        "executed program with {} cycles",
        report.total_instruction_count()
    );

    (duration, size(&proof), report.total_instruction_count())
}

pub fn benchmark_sha3(num_bytes: usize) -> (Duration, usize, u64) {
    let client = EnvProver::new();
    let (pk, vk) = client.setup(SHA3_ELF);

    let mut stdin = SP1Stdin::new();
    let input = vec![5u8; num_bytes];
    stdin.write(&input);

    println!("benchmark_sha3 start, num_bytes: {}", num_bytes);
    let start = Instant::now();
    let proof = client.prove(&pk, &stdin).run().unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!("benchmark_sha3 end, duration: {:?}", duration.as_secs_f64());

    client.verify(&proof, &vk).expect("verification failed");

    // Execute the program using the `ProverClient.execute` method, without generating a proof.
    let (_, report) = client.execute(SHA3_ELF, &stdin).run().unwrap();
    println!(
        "executed program with {} cycles",
        report.total_instruction_count()
    );

    (duration, size(&proof), report.total_instruction_count())
}

pub fn bench_fibonacci(n: u32) -> (Duration, usize, u64) {
    let client = EnvProver::new();
    let (pk, vk) = client.setup(FIBONACCI_ELF);

    let mut stdin = SP1Stdin::new();
    stdin.write(&n);

    println!("benchmark_fibonacci start, n: {}", n);
    let start = Instant::now();
    let proof = client.prove(&pk, &stdin).run().unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!(
        "benchmark_fibonacc end, duration: {:?}",
        duration.as_secs_f64()
    );

    client.verify(&proof, &vk).expect("verification failed");

    // Execute the program using the `ProverClient.execute` method, without generating a proof.
    let (_, report) = client.execute(FIBONACCI_ELF, &stdin).run().unwrap();
    println!(
        "executed program with {} cycles",
        report.total_instruction_count()
    );

    (duration, size(&proof), report.total_instruction_count())
}

pub fn bench_bigmem(value: u32) -> (Duration, usize, u64) {
    let client = EnvProver::new();
    let (pk, vk) = client.setup(BIGMEM_ELF);

    let mut stdin = SP1Stdin::new();
    stdin.write(&value);

    println!("benchmark_bigmem start, value: {}", value);
    let start = Instant::now();
    let proof = client.prove(&pk, &stdin).run().unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!(
        "benchmark_bigmem end, duration: {:?}",
        duration.as_secs_f64()
    );

    client.verify(&proof, &vk).expect("verification failed");

    // Execute the program using the `ProverClient.execute` method, without generating a proof.
    let (_, report) = client.execute(BIGMEM_ELF, &stdin).run().unwrap();
    println!(
        "executed program with {} cycles",
        report.total_instruction_count()
    );

    (duration, size(&proof), report.total_instruction_count())
}

pub fn benchmark_modpow(exp: u32) -> (Duration, usize, u64) {
    let client = EnvProver::new();
    let (pk, vk) = client.setup(MODPOW_ELF);

    let m =
        hex::decode("60908e3e36666b77de03caf7807ebe62e78d016aa5695ff5bc10b4fbbbe1f9cc").unwrap();
    let n = hex::decode("c5529f21f0afe6df78e83aab07b66c11ed9203af47a8ab9fdda4a83b4ae767720b833d2e150fcb4a4aec2776d0aa9762a3955d402b0c2665d3c4aa5db002656b65c75712eef82289d92bd6a3fba04d846e3680d1f9c0598a6717f07ae65400feb9d62156ecb37e0ef0c781299e300e268d825205ffad8892e267e63083348de4670907a8e23d4a03bc3a34496abb923fdf6181126cb073cf7a41620be431c7e1dc65e3d80a62fd76d04f8d011435529c7d683fc9f7c766c4527d3082b7dd2e5254876e1c8b296f41618c92cbb359b54df35010caa84286c35d7bf32c2fefd11c655fa48390c35d54274454a0ff749f8951fb23ee79a01e51a052716df0bc44db").unwrap();
    let mut stdin = SP1Stdin::new();
    stdin.write(&m);
    stdin.write(&exp);
    stdin.write(&n);

    // Execute the program using the `ProverClient.execute` method, without generating a proof.
    let (_, report) = client.execute(MODPOW_ELF, &stdin).run().unwrap();
    println!(
        "executed program with {} cycles",
        report.total_instruction_count()
    );

    println!("benchmark_modpow start, exp: {}", exp);
    let start = Instant::now();
    let proof = client.prove(&pk, &stdin).run().unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!(
        "benchmark_modpow end, duration: {:?}",
        duration.as_secs_f64()
    );

    client.verify(&proof, &vk).expect("verification failed");

    (duration, size(&proof), report.total_instruction_count())
}

pub fn benchmark_mul2048(iter: u32) -> (Duration, usize, u64) {
    let client = EnvProver::new();
    let (pk, vk) = client.setup(MUL2048_ELF);

    let a = hex::decode("edda500f613951a98a9051e5a009adba8d3d1fa1c3b876f14e51a8388158fb092716fd5b2a9bfae5eab90fa4d3c5a40bd5a4670ee3a1c6a609fbefd468e7d7fc61f172c7c4e7f40755530f46e17a0b6b052e20d5f342fa153766d19b718074f44e704ac31ad38a9256c9c63dd2712ba819298bd8b4979d7f3823680079bf627270df7255bccb6eef84ed51603eff86626cfbfee0e8bfa557e1f3e45388e855065036c0acd25b33b3b4073456216889d40ed21057af056e96121a0c903f96f024a2d9170b502fb371986c17807e7e6ec39e3277d7bd5f21cfc32c9c4e7e681baad6e73c4d0d58407ce7667e793de9c64128ed6aca993ec65f53339503420fd453").unwrap();
    let b = hex::decode("bf5d7af1eb193fb71b6728f31b0ba7f281e8135ce6d090ef940dad9b12630a191496853175073cbb70795680b0963ef5a1a6b6262ad6fdf126ed5d77073c52c0b113bc02f5c07dd940a9b54a3ded3e59ed6931d1ce1481615904dbb654d9958d3e1c747606a88105ef63095400df35fdd4a8ce892b4669f346036332a55bee61b5ae06b7743fd43619fb892e1fe0a9e66a77a7c4a72e3da46b277d364d5abfe71ef6d832a29ff0db4aa5803a6283436d93fef3ddbc5b9e409a261a60314b11fba27e1fdfadf104875b141e4406694e02a8daa10489efd3474bfac18da7547d8f1602c8723d24527bec8cb4f0e09bdbfe038267254920a73bde5b5ce4cd45e7da").unwrap();
    let mut stdin = SP1Stdin::new();
    stdin.write(&iter);
    stdin.write(&a);
    stdin.write(&b);

    // Execute the program using the `ProverClient.execute` method, without generating a proof.
    let (_, report) = client.execute(MUL2048_ELF, &stdin).run().unwrap();
    println!(
        "executed program with {} cycles",
        report.total_instruction_count()
    );

    println!("benchmark_mul2048 start, iter: {}", iter);
    let start = Instant::now();
    let proof = client.prove(&pk, &stdin).run().unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!(
        "benchmark_mul2048 end, duration: {:?}",
        duration.as_secs_f64()
    );

    client.verify(&proof, &vk).expect("verification failed");

    (duration, size(&proof), report.total_instruction_count())
}
