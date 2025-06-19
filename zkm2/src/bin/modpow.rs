use utils::benchmark_v2;
use zkm_script::{benchmark_modpow, init_logger};

fn main() {
    init_logger();
    let iters = [2, 3, 5];

    benchmark_v2(
        benchmark_modpow,
        &iters,
        "../benchmark_outputs/modpow_zkm2.csv",
        "iters",
    );
}
