use utils::benchmark_v2;
use sp1_script::{bench_fibonacci, init_logger};

fn main() {
    init_logger();

    let ns = [100, 1000, 10000, 50000];
    benchmark_v2(bench_fibonacci, &ns, "../benchmark_outputs/fibonacci_sp1.csv", "n");
}
