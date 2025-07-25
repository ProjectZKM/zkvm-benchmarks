use utils::benchmark_v2;
use zkm_script::{benchmark_poseidon2_permute, init_logger};

fn main() {
    init_logger();

    let iters = [1 << 10, 1 << 12, 1 << 14, 1 << 16, 1 << 18, 1 << 20];
    benchmark_v2(
        benchmark_poseidon2_permute,
        &iters,
        "../benchmark_outputs/poseidon2_permute_ziren.csv",
        "length",
    );
}
