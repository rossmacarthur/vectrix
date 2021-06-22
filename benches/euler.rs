// Benchmark euler vector operations.

use std::ops::{Add, AddAssign, Mul};

use criterion::{criterion_group, criterion_main, Criterion};
use rand::distributions::Standard;
use rand::prelude::*;
use rand_isaac::IsaacRng;
use vectrix::{Vector, Zero};

struct TestData<T, const M: usize> {
    acc: Vec<Vector<T, M>>,
    vel: Vec<Vector<T, M>>,
    pos: Vec<Vector<T, M>>,
}

impl<T, const M: usize> TestData<T, M>
where
    T: Copy + Zero + AddAssign + Add<Output = T> + Mul<Output = T>,
{
    fn random(size: usize) -> Self
    where
        Standard: Distribution<T>,
    {
        let mut rng = IsaacRng::seed_from_u64(0);
        Self {
            acc: vec![Vector::repeat_with(|| rng.gen()); size],
            vel: vec![Vector::zero(); size],
            pos: vec![Vector::zero(); size],
        }
    }

    #[allow(clippy::assign_op_pattern)]
    #[inline(always)]
    fn bench_euler_op(&mut self, update_rate: T) {
        for ((position, acceleration), velocity) in
            self.pos.iter_mut().zip(&self.acc).zip(&mut self.vel)
        {
            *velocity = *velocity + *acceleration * update_rate;
            *position = *position + *velocity * update_rate;
        }
    }

    #[inline(always)]
    fn bench_euler_op_assign(&mut self, update_rate: T) {
        for ((position, acceleration), velocity) in
            self.pos.iter_mut().zip(&self.acc).zip(&mut self.vel)
        {
            *velocity += *acceleration * update_rate;
            *position += *velocity * update_rate;
        }
    }
}

macro_rules! bench_euler {
    ($group:ident, $name:literal, Vector<$T:ty, $M:literal>, $meth:ident) => {{
        const SIZE: usize = 10_000;
        const UPDATE_RATE: $T = 1.0 / 60.0;
        let mut data = TestData::<$T, $M>::random(SIZE);
        $group.bench_function($name, |b| b.iter(|| data.$meth(UPDATE_RATE)));
    }};
}

macro_rules! bench_euler_op {
    ($group:ident, $name:literal, Vector<$T:ty, $M:literal>) => {{
        bench_euler!($group, $name, Vector<$T, $M>, bench_euler_op)
    }};
}

macro_rules! bench_euler_op_assign {
    ($group:ident, $name:literal, Vector<$T:ty, $M:literal>) => {{
        bench_euler!($group, $name, Vector<$T, $M>, bench_euler_op_assign)
    }};
}

fn bench_euler_2d(c: &mut Criterion) {
    let mut group = c.benchmark_group("vector/euler/2d");
    bench_euler_op!(group, "f32/op", Vector<f32, 2>);
    bench_euler_op!(group, "f64/op", Vector<f64, 2>);
    bench_euler_op_assign!(group, "f32/op_assign", Vector<f32, 2>);
    bench_euler_op_assign!(group, "f64/op_assign", Vector<f64, 2>);
}

fn bench_euler_3d(c: &mut Criterion) {
    let mut group = c.benchmark_group("vector/euler/3d");
    bench_euler_op!(group, "f32/op", Vector<f32, 3>);
    bench_euler_op!(group, "f64/op", Vector<f64, 3>);
    bench_euler_op_assign!(group, "f32/op_assign", Vector<f32, 3>);
    bench_euler_op_assign!(group, "f64/op_assign", Vector<f64, 3>);
}

criterion_group!(benches, bench_euler_2d, bench_euler_3d);
criterion_main! {benches}
