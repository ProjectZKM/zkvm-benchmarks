use utils::benchmark_v2;
use zkm_script::{benchmark_poseidon2, init_logger};

fn main() {
    init_logger();

    let lengths = [32, 256, 512, 1024, 2048];
    benchmark_v2(
        benchmark_poseidon2,
        &lengths,
        "../benchmark_outputs/poseidon2_ziren.csv",
        "byte length",
    );
}
