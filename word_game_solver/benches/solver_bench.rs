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

/*
I'm keeping this benchmark around just in case, but commenting it out because it would require
making the underlying function public. The function is pretty simple, and my one optimization idea
(binary search) didn't help.

fn bench_num_matches(c: &mut Criterion) {
    c.bench_function("num_matches", |b| {
        b.iter(|| {
            num_matches(&['a', 'b', 'c', 'd', 'e'], &['a', 'c', 'e', 'f', 'g']);
            num_matches(&['a', 'b', 'c', 'd', 'e'], &['a', 'c', 'f', 'g', 'h']);
            num_matches(&['a', 'c', 'd', 'e', 'z'], &['a', 'b', 'e', 'f', 'g']);
        })
    });
}
*/

criterion_group!(benches, bench_is_unique);
criterion_main!(benches);
