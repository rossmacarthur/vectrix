use std::iter;

use criterion::{criterion_group, Criterion};
use rand::distributions::Standard;
use rand::prelude::*;

use vectrs::traits::*;
use vectrs::Vector;

fn random_vector<T: Base, const N: usize>() -> Vector<T, N>
where
    Standard: Distribution<T>,
{
    iter::repeat_with(random).collect()
}

////////////////////////////////////////////////////////////////////////////////
// Euler
////////////////////////////////////////////////////////////////////////////////

macro_rules! bench_euler_op {
    ($group:ident, $name:literal, Vector<$ty:ty, $size:literal>) => {{
        const SIZE: usize = 10_000;
        const UPDATE_RATE: $ty = 1.0 / 60.0;

        struct TestData {
            acc: Vec<Vector<$ty, $size>>,
            vel: Vec<Vector<$ty, $size>>,
            pos: Vec<Vector<$ty, $size>>,
        }

        let mut data = TestData {
            acc: vec![random_vector(); SIZE],
            vel: vec![Vector::zero(); SIZE],
            pos: vec![Vector::zero(); SIZE],
        };

        $group.bench_function($name, |b| {
            b.iter(|| {
                for ((position, acceleration), velocity) in
                    data.pos.iter_mut().zip(&data.acc).zip(&mut data.vel)
                {
                    *velocity = *velocity + *acceleration * UPDATE_RATE;
                    *position = *position + *velocity * UPDATE_RATE;
                }
            })
        });
    }};
}

macro_rules! bench_euler_op_assign {
    ($group:ident, $name:literal, Vector<$ty:ty, $size:literal>) => {{
        const SIZE: usize = 10_000;
        const UPDATE_RATE: $ty = 1.0 / 60.0;

        struct TestData {
            acc: Vec<Vector<$ty, $size>>,
            vel: Vec<Vector<$ty, $size>>,
            pos: Vec<Vector<$ty, $size>>,
        }

        let mut data = TestData {
            acc: vec![random_vector(); SIZE],
            vel: vec![Vector::zero(); SIZE],
            pos: vec![Vector::zero(); SIZE],
        };

        $group.bench_function($name, |b| {
            b.iter(|| {
                for ((position, acceleration), velocity) in
                    data.pos.iter_mut().zip(&data.acc).zip(&mut data.vel)
                {
                    *velocity += *acceleration * UPDATE_RATE;
                    *position += *velocity * UPDATE_RATE;
                }
            })
        });
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
criterion::criterion_main! {benches}
