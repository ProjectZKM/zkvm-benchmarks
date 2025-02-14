use utils::benchmark;
use zkm_script::{benchmark_fibonacci, init_logger};

fn main() {
    init_logger();

    let ns = [100, 1000, 10000, 50000];
    benchmark(benchmark_fibonacci, &ns, "../benchmark_outputs/fiboancci_zkm.csv", "n");
}
