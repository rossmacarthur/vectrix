use criterion::{criterion_group, Criterion};

use vectrs::Vector;

fn bench_vector_add(c: &mut Criterion) {
    let vector1: Vector<i64, 12> = Vector::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    let vector2: Vector<i64, 12> = Vector::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    c.bench_function("vector_add", |b| b.iter(|| vector1 + vector2));
}

fn bench_vector_abs(c: &mut Criterion) {
    let vector: Vector<i64, 12> = Vector::from([-1, 2, -3, 4, -5, 6, -7, 8, -9, 10, -11, 12]);
    c.bench_function("vector_abs", |b| b.iter(|| vector.abs()));
}

fn bench_vector_l1_norm(c: &mut Criterion) {
    let vector: Vector<i64, 12> = Vector::from([-1, 2, -3, 4, -5, 6, -7, 8, -9, 10, -11, 12]);
    c.bench_function("vector_l1_norm", |b| b.iter(|| vector.l1_norm()));
}

criterion_group!(
    benches,
    bench_vector_add,
    bench_vector_abs,
    bench_vector_l1_norm
);
criterion::criterion_main! {benches}
