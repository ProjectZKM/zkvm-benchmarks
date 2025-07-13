use utils::benchmark_v2;
use zkm_script::{benchmark_mul2048, init_logger};

fn main() {
    init_logger();
    let iters = [1, 2, 4, 8, 16, 32, 64, 128, 256];

    benchmark_v2(
        benchmark_mul2048,
        &iters,
        "../benchmark_outputs/mul2048_ziren.csv",
        "iters",
    );
}
