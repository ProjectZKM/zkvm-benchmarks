use utils::benchmark_v2;
use zkm_script::{bench_tendermint, init_logger};

fn main() {
    init_logger();

    let values = [2279100u32];
    benchmark_v2(
        bench_tendermint,
        &values,
        "../benchmark_outputs/tendermint_zkm2.csv",
        "block",
    );
}
