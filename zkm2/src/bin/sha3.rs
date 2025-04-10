use utils::benchmark_v2;
use zkm_script::{benchmark_sha3, init_logger};

fn main() {
    init_logger();

    let lengths = [32, 256, 512, 1024, 2048];
    benchmark_v2(benchmark_sha3, &lengths, "../benchmark_outputs/sha3_zkm2.csv", "byte length");
}
