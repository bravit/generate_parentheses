use criterion::{Criterion, criterion_group, criterion_main};
use gp_algo::implementations::IMPLEMENTATIONS;

pub fn criterion_benchmark(c: &mut Criterion) {
    for size in [5, 10, 12] {
        for (name, fun) in IMPLEMENTATIONS {
            c.bench_function(format!("{}, n = {}", name, size).as_str(),
                             |b| b.iter(|| fun(size)));
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
