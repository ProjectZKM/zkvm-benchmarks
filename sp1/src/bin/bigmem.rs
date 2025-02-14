use utils::benchmark;
use sp1_script::{bench_bigmem, init_logger};

fn main() {
    init_logger();

    let values = [5u32];
    benchmark(bench_bigmem, &values, "../benchmark_outputs/bigmem_sp1.csv", "value");
}
