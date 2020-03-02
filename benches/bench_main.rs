mod benchmarks;

use criterion::Criterion;

#[tokio::main]
async fn main() {
    benchmarks::open_latex::benches().await;
    Criterion::default().configure_from_args().final_summary();
}
