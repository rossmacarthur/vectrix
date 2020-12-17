//! Overloaded operators for a vector.

use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

use crate::prelude::*;

macro_rules! impl_add {
    ($lhs:ty, $rhs:ty, $output:ty) => {
        impl<T, const N: usize> Add<$rhs> for $lhs
        where
            T: Copy + Zero + Add<Output = T>,
        {
            type Output = $output;

            fn add(self, other: $rhs) -> Self::Output {
                let mut vector = Vector::default();
                for i in 0..N {
                    vector.inner[i] = self.inner[i] + other.inner[i];
                }
                vector
            }
        }
    };
}

macro_rules! impl_sub {
    ($lhs:ty, $rhs:ty, $output:ty) => {
        impl<T, const N: usize> Sub<$rhs> for $lhs
        where
            T: Copy + Zero + Sub<Output = T>,
        {
            type Output = $output;

            fn sub(self, other: $rhs) -> Self::Output {
                let mut vector = Vector::default();
                for i in 0..N {
                    vector.inner[i] = self.inner[i] - other.inner[i];
                }
                vector
            }
        }
    };
}

macro_rules! impl_mul {
    ($lhs:ty, $rhs:ty, $output:ty) => {
        impl<'a, T, const N: usize> Mul<$rhs> for $lhs
        where
            T: Copy + Zero + Mul<$rhs, Output = T>,
        {
            type Output = $output;

            fn mul(self, other: $rhs) -> Self::Output {
                let mut vector = Vector::default();
                for i in 0..N {
                    vector.inner[i] = self.inner[i] * other;
                }
                vector
            }
        }
    };
}

macro_rules! impl_add_assign {
    ($self:ty, $other:ty) => {
        impl<T, const N: usize> AddAssign<$other> for $self
        where
            T: Copy + AddAssign,
        {
            fn add_assign(&mut self, other: $other) {
                for i in 0..N {
                    self.inner[i] += other.inner[i]
                }
            }
        }
    };
}

macro_rules! impl_sub_assign {
    ($self:ty, $other:ty) => {
        impl<T, const N: usize> SubAssign<$other> for $self
        where
            T: Copy + SubAssign,
        {
            fn sub_assign(&mut self, other: $other) {
                for i in 0..N {
                    self.inner[i] -= other.inner[i]
                }
            }
        }
    };
}

impl_add!(Vector<T, N>, Vector<T, N>, Vector<T, N>);
impl_add!(Vector<T, N>, &Vector<T, N>, Vector<T, N>);
impl_add!(&Vector<T, N>, &Vector<T, N>, Vector<T, N>);

impl_sub!(Vector<T, N>, Vector<T, N>, Vector<T, N>);
impl_sub!(Vector<T, N>, &Vector<T, N>, Vector<T, N>);
impl_sub!(&Vector<T, N>, &Vector<T, N>, Vector<T, N>);

impl_mul!(Vector<T, N>, T, Vector<T, N>);
impl_mul!(Vector<T, N>, &'a T, Vector<T, N>);
impl_mul!(&Vector<T, N>, T, Vector<T, N>);
impl_mul!(&Vector<T, N>, &'a T, Vector<T, N>);

impl_add_assign!(Vector<T, N>, Vector<T, N>);
impl_add_assign!(Vector<T, N>, &Vector<T, N>);

impl_sub_assign!(Vector<T, N>, Vector<T, N>);
impl_sub_assign!(Vector<T, N>, &Vector<T, N>);
