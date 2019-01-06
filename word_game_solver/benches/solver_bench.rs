#[macro_use]
extern crate criterion;

use criterion::Criterion;

extern crate solver_lib;
use solver_lib::*;

fn bench_is_unique(c: &mut Criterion) {
    c.bench_function("is_unique", |b| {
        b.iter(|| {
            is_unique("abcde");
            !is_unique("abcce")
        });
    });
}

criterion_group!(benches, bench_is_unique);
criterion_main!(benches);
