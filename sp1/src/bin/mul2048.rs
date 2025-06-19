use sp1_script::{benchmark_mul2048, init_logger};
use utils::benchmark_v2;

fn main() {
    init_logger();
    let iters = [1, 2, 4, 8, 16, 32, 64, 128, 256];

    benchmark_v2(
        benchmark_mul2048,
        &iters,
        "../benchmark_outputs/mul2048_sp1.csv",
        "iters",
    );
}
