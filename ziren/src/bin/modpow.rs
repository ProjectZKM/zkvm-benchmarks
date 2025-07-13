use utils::benchmark_v2;
use zkm_script::{benchmark_modpow, init_logger};

fn main() {
    init_logger();
    let exp = [257, 521, 1031, 2053, 4099, 8209, 16411, 32771];

    benchmark_v2(
        benchmark_modpow,
        &exp,
        "../benchmark_outputs/modpow_ziren.csv",
        "exp",
    );
}
