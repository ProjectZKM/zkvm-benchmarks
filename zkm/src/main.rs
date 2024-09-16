use std::time::{Duration, Instant};

use utils::benchmark;

use std::fs::File;
use std::io::BufReader;
use std::ops::Range;

use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
use plonky2::util::timing::TimingTree;
use plonky2x::backend::circuit::Groth16WrapperParameters;
use plonky2x::backend::wrapper::wrap::WrappedCircuit;
use plonky2x::frontend::builder::CircuitBuilder as WrapperBuilder;
use plonky2x::prelude::DefaultParameters;

use zkm_emulator::utils::{load_elf_with_patch, split_prog_into_segs};
use zkm_prover::all_stark::AllStark;
use zkm_prover::config::StarkConfig;
use zkm_prover::cpu::kernel::assembler::segment_kernel;
use zkm_prover::fixed_recursive_verifier::AllRecursiveCircuits;
use zkm_prover::proof;
use zkm_prover::proof::PublicValues;
use zkm_prover::prover::prove;
use zkm_prover::verifier::verify_proof;

const FIBONACCI_ELF: &str = "./fibonacci/target/mips-unknown-linux-musl/debug/fibonacci";
//const SHA2_ELF: &str = "./sha2/target/mips-unknown-linux-musl/debug/sha2-bench";
const SHA2_ELF: &str = "./sha2/target/mips-unknown-linux-musl/release/sha2-bench";
const SHA2_CHAIN_ELF: &str = "./sha2-chain/target/mips-unknown-linux-musl/debug/sha2-chain";
const SHA3_CHAIN_ELF: &str = "./sha3-chain/target/mips-unknown-linux-musl/debug/sha3-chain";
const SHA3_ELF: &str = "./sha3/target/mips-unknown-linux-musl/debug/sha3-bench";
const BIGMEM_ELF: &str = "./bigmem/target/mips-unknown-linux-musl/debug/bigmem";
//const SEG_SIZE: usize = 4194304;
const SEG_SIZE: usize = 262144; //G

const DEGREE_BITS_RANGE: [Range<usize>; 6] = [10..21, 12..22, 12..21, 8..21, 6..21, 13..23];


fn main() {
    init_logger();

    let _ = std::fs::remove_dir_all("/tmp/zkm.old");
    let _ = std::fs::rename("/tmp/zkm", "/tmp/zkm.old");

    let lengths = [32, 256, 512, 1024, 2048];
    benchmark(benchmark_sha2, &lengths, "../benchmark_outputs/sha2_zkm.csv", "byte length");
   /* benchmark(benchmark_sha3, &lengths, "../benchmark_outputs/sha3_zkm.csv", "byte length");

    let ns = [100, 1000, 10000, 50000];
    benchmark(benchmark_fibonacci, &ns, "../benchmark_outputs/fiboancci_zkm.csv", "n");

    let values = [5];
    benchmark(benchmark_bigmem, &values, "../benchmark_outputs/bigmem_zkm.csv", "value");

    let iters = [230, 460, 920, 1840, 3680];
    benchmark(benchmark_sha2_chain, &iters, "../benchmark_outputs/sha2_chain_zkm.csv", "iters");
    benchmark(benchmark_sha3_chain, &iters, "../benchmark_outputs/sha3_chain_zkm.csv", "iters"); */
}

fn prove_single_seg_common(
    seg_file: &str,
    basedir: &str,
    block: &str,
    file: &str,
    seg_size: usize,
) -> usize {
    let seg_reader = BufReader::new(File::open(seg_file).unwrap());
    let kernel = segment_kernel(basedir, block, file, seg_reader, seg_size);

    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    let allstark: AllStark<F, D> = AllStark::default();
    let config = StarkConfig::standard_fast_config();
    let mut timing = TimingTree::new("prove", log::Level::Info);
    let allproof: proof::AllProof<GoldilocksField, C, D> =
        prove(&allstark, &kernel, &config, &mut timing).unwrap();
    let mut count_bytes: usize = 0;
    for (row, proof) in allproof.stark_proofs.clone().iter().enumerate() {
        let proof_str = serde_json::to_string(&proof.proof).unwrap();
        log::info!("row:{} proof bytes:{}", row, proof_str.len());
        count_bytes += proof_str.len();
    }
    timing.filter(Duration::from_millis(100)).print();
    log::info!("total proof bytes:{}KB", count_bytes / 1024);
    verify_proof(&allstark, allproof, &config).unwrap();
    log::info!("Prove done");
    count_bytes
}

fn prove_multi_seg_common(
    seg_dir: &str,
    basedir: &str,
    block: &str,
    file: &str,
    seg_size: usize,
    seg_file_number: usize,
    seg_start_id: usize,
) -> usize {
    type InnerParameters = DefaultParameters;
    type OuterParameters = Groth16WrapperParameters;

    type F = GoldilocksField;
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;

    if seg_file_number < 2 {
        panic!("seg file number must >= 2\n");
    }

    let total_timing = TimingTree::new("prove total time", log::Level::Info);
    let all_stark = AllStark::<F, D>::default();
    let config = StarkConfig::standard_fast_config();
    // Preprocess all circuits.
    let all_circuits =
        AllRecursiveCircuits::<F, C, D>::new(&all_stark, &DEGREE_BITS_RANGE, &config);

    let seg_file = format!("{}/{}", seg_dir, seg_start_id);
    log::info!("Process segment {}", seg_file);
    let seg_reader = BufReader::new(File::open(seg_file).unwrap());
    let input_first = segment_kernel(basedir, block, file, seg_reader, seg_size);
    let mut timing = TimingTree::new("prove root first", log::Level::Info);
    let (mut agg_proof, mut updated_agg_public_values) =
        all_circuits.prove_root(&all_stark, &input_first, &config, &mut timing).unwrap();

    timing.filter(Duration::from_millis(100)).print();
    all_circuits.verify_root(agg_proof.clone()).unwrap();

    let mut base_seg = seg_start_id + 1;
    let mut seg_num = seg_file_number - 1;
    let mut is_agg = false;

    if seg_file_number % 2 == 0 {
        let seg_file = format!("{}/{}", seg_dir, seg_start_id + 1);
        log::info!("Process segment {}", seg_file);
        let seg_reader = BufReader::new(File::open(seg_file).unwrap());
        let input = segment_kernel(basedir, block, file, seg_reader, seg_size);
        timing = TimingTree::new("prove root second", log::Level::Info);
        let (root_proof, public_values) =
            all_circuits.prove_root(&all_stark, &input, &config, &mut timing).unwrap();
        timing.filter(Duration::from_millis(100)).print();

        all_circuits.verify_root(root_proof.clone()).unwrap();

        // Update public values for the aggregation.
        let agg_public_values = PublicValues {
            roots_before: updated_agg_public_values.roots_before,
            roots_after: public_values.roots_after,
            userdata: public_values.userdata,
        };
        timing = TimingTree::new("prove aggression", log::Level::Info);
        // We can duplicate the proofs here because the state hasn't mutated.
        (agg_proof, updated_agg_public_values) = all_circuits.prove_aggregation(
            false,
            &agg_proof,
            false,
            &root_proof,
            agg_public_values.clone(),
        ).unwrap();
        timing.filter(Duration::from_millis(100)).print();
        all_circuits.verify_aggregation(&agg_proof).unwrap();

        is_agg = true;
        base_seg = seg_start_id + 2;
        seg_num -= 1;
    }

    for i in 0..seg_num / 2 {
        let seg_file = format!("{}/{}", seg_dir, base_seg + (i << 1));
        log::info!("Process segment {}", seg_file);
        let seg_reader = BufReader::new(File::open(&seg_file).unwrap());
        let input_first = segment_kernel(basedir, block, file, seg_reader, seg_size);
        let mut timing = TimingTree::new("prove root first", log::Level::Info);
        let (root_proof_first, first_public_values) =
            all_circuits.prove_root(&all_stark, &input_first, &config, &mut timing).unwrap();

        timing.filter(Duration::from_millis(100)).print();
        all_circuits.verify_root(root_proof_first.clone()).unwrap();

        let seg_file = format!("{}/{}", seg_dir, base_seg + (i << 1) + 1);
        log::info!("Process segment {}", seg_file);
        let seg_reader = BufReader::new(File::open(&seg_file).unwrap());
        let input = segment_kernel(basedir, block, file, seg_reader, seg_size);
        let mut timing = TimingTree::new("prove root second", log::Level::Info);
        let (root_proof, public_values) =
            all_circuits.prove_root(&all_stark, &input, &config, &mut timing).unwrap();
        timing.filter(Duration::from_millis(100)).print();

        all_circuits.verify_root(root_proof.clone()).unwrap();

        // Update public values for the aggregation.
        let new_agg_public_values = PublicValues {
            roots_before: first_public_values.roots_before,
            roots_after: public_values.roots_after,
            userdata: public_values.userdata,
        };
        timing = TimingTree::new("prove aggression", log::Level::Info);
        // We can duplicate the proofs here because the state hasn't mutated.
        let (new_agg_proof, new_updated_agg_public_values) = all_circuits.prove_aggregation(
            false,
            &root_proof_first,
            false,
            &root_proof,
            new_agg_public_values,
        ).unwrap();
        timing.filter(Duration::from_millis(100)).print();
        all_circuits.verify_aggregation(&new_agg_proof).unwrap();

        // Update public values for the nested aggregation.
        let agg_public_values = PublicValues {
            roots_before: updated_agg_public_values.roots_before,
            roots_after: new_updated_agg_public_values.roots_after,
            userdata: new_updated_agg_public_values.userdata,
        };
        timing = TimingTree::new("prove nested aggression", log::Level::Info);

        // We can duplicate the proofs here because the state hasn't mutated.
        (agg_proof, updated_agg_public_values) = all_circuits.prove_aggregation(
            is_agg,
            &agg_proof,
            true,
            &new_agg_proof,
            agg_public_values.clone(),
        ).unwrap();
        is_agg = true;
        timing.filter(Duration::from_millis(100)).print();

        all_circuits.verify_aggregation(&agg_proof).unwrap();
    }

    let (block_proof, _block_public_values) =
        all_circuits.prove_block(None, &agg_proof, updated_agg_public_values).unwrap();

    let size = serde_json::to_string(&block_proof.proof).unwrap().len();
    log::info!(
        "proof size: {:?}",
        size
    );
    /*
    let _result = all_circuits.verify_block(&block_proof);

    let build_path = "verifier/data".to_string();
    let path = format!("{}/test_circuit/", build_path);
    let builder = WrapperBuilder::<DefaultParameters, 2>::new();
    let mut circuit = builder.build();
    circuit.set_data(all_circuits.block.circuit);
    let mut bit_size = vec![32usize; 16];
    bit_size.extend(vec![8; 32]);
    bit_size.extend(vec![64; 68]);
    let wrapped_circuit = WrappedCircuit::<InnerParameters, OuterParameters, D>::build(
        circuit,
        Some((vec![], bit_size)),
    );
    log::info!("build finish");

    let wrapped_proof = wrapped_circuit.prove(&block_proof).unwrap();
    wrapped_proof.save(path).unwrap();

    total_timing.filter(Duration::from_millis(100)).print();
    */
    size
}

fn init_logger() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init()
}

fn benchmark_sha2_chain(iters: u32) -> (Duration, usize) {
    let input = [5u8; 32];
    let mut state = load_elf_with_patch(SHA2_CHAIN_ELF, vec![]);
    state.add_input_stream(&input);
    state.add_input_stream(&iters);
    
    let seg_size = SEG_SIZE;
    let seg_path = "/tmp/zkm/sha2-chain";

    let (total_steps, mut state) = split_prog_into_segs(state, seg_path, "", seg_size);

    let mut seg_num = 1usize;
    if seg_size != 0 {
        seg_num = (total_steps + seg_size - 1) / seg_size;
    }

    let start = Instant::now();
    let size = if seg_num == 1 {
        let seg_file = format!("{seg_path}/{}", 0);
        prove_single_seg_common(&seg_file, "", "", "", total_steps)
    } else {
        prove_multi_seg_common(seg_path, "", "", "", seg_size, seg_num, 0)
    };

    let end = Instant::now();
    let duration = end.duration_since(start);

    let _hash =  state.read_public_values::<[u8; 32]>();

    (duration, size)
}

fn benchmark_sha3_chain(iters: u32) -> (Duration, usize) {
    let input = [5u8; 32];
    let mut state = load_elf_with_patch(SHA3_CHAIN_ELF, vec![]);
    state.add_input_stream(&input);
    state.add_input_stream(&iters);
    
    let seg_size = SEG_SIZE;
    let seg_path = "/tmp/zkm/sha3-chain";

    let (total_steps, mut state) = split_prog_into_segs(state, seg_path, "", seg_size);

    let mut seg_num = 1usize;
    if seg_size != 0 {
        seg_num = (total_steps + seg_size - 1) / seg_size;
    }

    let start = Instant::now();
    let size = if seg_num == 1 {
        let seg_file = format!("{seg_path}/{}", 0);
        prove_single_seg_common(&seg_file, "", "", "", total_steps)
    } else {
        prove_multi_seg_common(seg_path, "", "", "", seg_size, seg_num, 0)
    };

    let end = Instant::now();

    let duration = end.duration_since(start);

    let _hash =  state.read_public_values::<[u8; 32]>();

    (duration, size)
}

fn benchmark_sha2(num_bytes: usize) -> (Duration, usize) {
    let input = vec![5u8; num_bytes];
    let mut state = load_elf_with_patch(SHA2_ELF, vec![]);
    state.add_input_stream(&input);
    
    let seg_size = SEG_SIZE;
    let seg_path = "/tmp/zkm/sha2";

    let (total_steps, mut state) = split_prog_into_segs(state, seg_path, "", seg_size);

    let mut seg_num = 1usize;
    if seg_size != 0 {
        seg_num = (total_steps + seg_size - 1) / seg_size;
    }

    let start = Instant::now();
    let size = if seg_num == 1 {
        let seg_file = format!("{seg_path}/{}", 0);
        prove_single_seg_common(&seg_file, "", "", "", total_steps)
    } else {
        prove_multi_seg_common(seg_path, "", "", "", seg_size, seg_num, 0)
    };

    let end = Instant::now();
    let duration = end.duration_since(start);

    let _hash =  state.read_public_values::<[u8; 32]>();

    (duration, size)
}

fn benchmark_sha3(num_bytes: usize) -> (Duration, usize) {
    let input = vec![5u8; num_bytes];
    let mut state = load_elf_with_patch(SHA3_ELF, vec![]);
    state.add_input_stream(&input);
    
    let seg_size = SEG_SIZE;
    let seg_path = "/tmp/zkm/sha3";

    let (total_steps, mut state) = split_prog_into_segs(state, seg_path, "", seg_size);

    let mut seg_num = 1usize;
    if seg_size != 0 {
        seg_num = (total_steps + seg_size - 1) / seg_size;
    }

    let start = Instant::now();
    let size = if seg_num == 1 {
        let seg_file = format!("{seg_path}/{}", 0);
        prove_single_seg_common(&seg_file, "", "", "", total_steps)
    } else {
        prove_multi_seg_common(seg_path, "", "", "", seg_size, seg_num, 0)
    };
    let end = Instant::now();
    let duration = end.duration_since(start);

    let _hash =  state.read_public_values::<[u8; 32]>();

    (duration, size)
}

fn benchmark_fibonacci(n: u32) -> (Duration, usize) {
    let mut state = load_elf_with_patch(FIBONACCI_ELF, vec![]);
    state.add_input_stream(&n);

    let seg_size = SEG_SIZE;
    let seg_path = "/tmp/zkm/fibonacci";

    let (total_steps, mut state) = split_prog_into_segs(state, seg_path, "", seg_size);

    let mut seg_num = 1usize;
    if seg_size != 0 {
        seg_num = (total_steps + seg_size - 1) / seg_size;
    }

    let start = Instant::now();
    let size = if seg_num == 1 {
        let seg_file = format!("{seg_path}/{}", 0);
        prove_single_seg_common(&seg_file, "", "", "", total_steps)
    } else {
        prove_multi_seg_common(seg_path, "", "", "", seg_size, seg_num, 0)
    };
    let end = Instant::now();
    let duration = end.duration_since(start);

    let _output = state.read_public_values::<u128>();
    (duration, size)
}

fn benchmark_bigmem(value: u32) -> (Duration, usize) {
    let mut state = load_elf_with_patch(BIGMEM_ELF, vec![]);
    state.add_input_stream(&value);

    let seg_size = SEG_SIZE;
    let seg_path = "/tmp/zkm/bigmem";

    let (total_steps, mut state) = split_prog_into_segs(state, seg_path, "", seg_size);

    let mut seg_num = 1usize;
    if seg_size != 0 {
        seg_num = (total_steps + seg_size - 1) / seg_size;
    }

    let start = Instant::now();
    let size = if seg_num == 1 {
        let seg_file = format!("{seg_path}/{}", 0);
        prove_single_seg_common(&seg_file, "", "", "", total_steps)
    } else {
        prove_multi_seg_common(seg_path, "", "", "", seg_size, seg_num, 0)
    };
    let end = Instant::now();
    let duration = end.duration_since(start);

    let _output = state.read_public_values::<u32>();
    (duration, size)
}
