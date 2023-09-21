use crate::algorithm::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// TODO: implement eval
fn eval() {}

fn eval_benchmark(c: &mut Criterion) {
    c.bench_function("eval", |b| b.iter(|| black_box(eval())));
}

criterion_group!(benches, eval_benchmark);
criterion_main!(benches);
