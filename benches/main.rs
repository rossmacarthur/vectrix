use std::iter;

use criterion::{criterion_group, Criterion};
use rand::prelude::*;

use vectrs::Vector;

////////////////////////////////////////////////////////////////////////////////
// Operators
////////////////////////////////////////////////////////////////////////////////

fn bench_vector_add(c: &mut Criterion) {
    let vector1: Vector<i64, 12> = iter::repeat_with(random).collect();
    let vector2: Vector<i64, 12> = iter::repeat_with(random).collect();
    c.bench_function("vector_add", |b| b.iter(|| vector1 + vector2));
}

////////////////////////////////////////////////////////////////////////////////
// Iterators
////////////////////////////////////////////////////////////////////////////////

fn bench_vector_collect(c: &mut Criterion) {
    let vector: Vector<i64, 12> = iter::repeat_with(random).collect();
    c.bench_function("vector_collect", |b| {
        b.iter(|| {
            let _vector = vector.into_iter().collect::<Vector<i64, 12>>();
        })
    });
}

////////////////////////////////////////////////////////////////////////////////
// General methods
////////////////////////////////////////////////////////////////////////////////

fn bench_vector_abs(c: &mut Criterion) {
    let vector: Vector<i64, 12> = iter::repeat_with(random).collect();
    c.bench_function("vector_abs", |b| b.iter(|| vector.abs()));
}

fn bench_vector_dot(c: &mut Criterion) {
    let vector1: Vector<i64, 12> = iter::repeat_with(random).collect();
    let vector2: Vector<i64, 12> = iter::repeat_with(random).collect();
    c.bench_function("vector_dot", |b| b.iter(|| vector1.dot(&vector2)));
}

fn bench_vector_l1_norm(c: &mut Criterion) {
    let vector: Vector<i64, 12> = iter::repeat_with(random).collect();
    c.bench_function("vector_l1_norm", |b| b.iter(|| vector.l1_norm()));
}

criterion_group!(
    benches,
    bench_vector_add,
    bench_vector_collect,
    bench_vector_abs,
    bench_vector_dot,
    bench_vector_l1_norm
);
criterion::criterion_main! {benches}
